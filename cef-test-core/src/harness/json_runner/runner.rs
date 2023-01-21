use std::path::{Path, PathBuf};

use super::super::{Instruction, Feedback, TestRunner, TestMetadata};
use super::{Test, FailureMode, TestStepType};

/// Test runner for specific JSON data structures.
pub struct JsonRunner {
    test_data: Test,
    step_i: usize,
    op_i: usize,
    success: bool,
    filepath: Option<PathBuf>,
    current_macro: Option<Box<Self>>,
}

impl JsonRunner {
    /// Load test information from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = std::io::BufReader::new(std::fs::File::open(path.as_ref())?);
        let test = serde_json::from_reader(file)?;
        Ok(Self {
            test_data: test,
            step_i: 0,
            op_i: 0,
            success: true,
            filepath: Some(path.as_ref().into()),
            current_macro: None,
        })
    }

    /// Construct JsonRunner in memory
    pub fn new(test: Test) -> Self {
        Self {
            test_data: test,
            step_i: 0,
            op_i: 0,
            success: true,
            filepath: None,
            current_macro: None,
        }
    }
}

impl TestRunner for JsonRunner {
    fn next(&mut self, feedback: Feedback) -> Option<Instruction> {
        self.success = feedback.is_ok();
        let fail_mode = self.test_data.info.fail_mode.clone();
        if matches!(fail_mode, FailureMode::FastFail) && !feedback.is_ok() {
            return None;
        }
        // handle in-progress macro execution
        if let Some(current_macro) = &mut self.current_macro {
            if let Some(instr) = current_macro.next(feedback.clone()) {
                return Some(instr);
            } else {
                self.current_macro = None;
            }
        }
        #[allow(clippy::never_loop)]
        'step_loop: while self.step_i < self.test_data.test.len() {
            let step = &self.test_data.test[self.step_i];
            match step {
                TestStepType::Regular(step) => {
                    'op_loop: while self.op_i < step.operations.len() {
                        if matches!(fail_mode, FailureMode::SkipInstructions) && !feedback.is_ok() {
                            log::info!("{:?} Failing instruction, going to next step", fail_mode);
                            break 'op_loop;
                        }
                        let instruction = &step.operations[self.op_i];
                        log::debug!("Performing step {}, operation {}", self.step_i, self.op_i);
                        self.op_i += 1;
                        return Some(instruction.clone().into_instruction(step.tab.clone()));
                    }
                },
                TestStepType::Macro { name, file } => {
                    let path = if let Some(filepath) = &self.filepath {
                        filepath.join(file)
                    } else {
                        file.to_owned()
                    };
                    log::debug!("Macro {} expanded to {}", name, path.display());
                    self.step_i += 1;
                    match Self::from_file(&path) {
                        Ok(mut new_macro) => {
                            if let Some(instr) = new_macro.next(feedback.clone()) {
                                self.current_macro = Some(Box::new(new_macro));
                                return Some(instr);
                            } else {
                                log::warn!("Macro {} ({}) is empty!", name, file.display());
                            }
                        },
                        Err(e) => log::error!("Macro {} ({}) failed to load: {}", name, file.display(), e),
                    }
                }
            }
            if matches!(fail_mode, FailureMode::SkipSteps) && !self.success {
                log::info!("{:?} Failing step complete, ending test", fail_mode);
                break 'step_loop;
            }
            self.op_i = 0;
            self.step_i += 1;
        }
        None
    }

    fn meta(&self) -> TestMetadata {
        let mut metadata: TestMetadata = self.test_data.info.clone().into();
        metadata.success = self.success;
        metadata
    }
}
