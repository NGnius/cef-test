use super::super::{Instruction, Feedback, TestRunner, TestMetadata};
use super::Test;

/// Test runner for specific JSON data structures.
pub struct JsonRunner {
    test_data: Test,
    success: bool,
}

impl JsonRunner {
    /// Load test information from file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let file = std::io::BufReader::new(std::fs::File::open(path.as_ref())?);
        let test = serde_json::from_reader(file)?;
        Ok(Self {
            test_data: test,
            success: true,
        })
    }

    /// Construct JsonRunner in memory
    pub fn new(test: Test) -> Self {
        Self {
            test_data: test,
            success: true,
        }
    }
}

impl TestRunner for JsonRunner {
    fn next(&mut self, feedback: Feedback) -> Option<Instruction> {
        // TODO
        self.success = feedback.is_ok();
        log::error!("JsonRunner.next(...) is UNIMPLEMENTED!");
        None
    }

    fn meta(&self) -> TestMetadata {
        let mut metadata: TestMetadata = self.test_data.info.clone().into();
        metadata.success = self.success;
        metadata
    }
}
