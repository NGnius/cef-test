use super::{TestRunner, TestAdaptor, TestMetadata};
use super::{Instruction, TestAssert, TestOp, Feedback};

/// Harness which runs one or more tests
pub struct TestHarness<R: TestRunner, A: TestAdaptor> {
    tests: Vec<R>,
    adaptor: A,
}

impl<R: TestRunner, A: TestAdaptor> TestHarness<R, A> {
    /// Construct a new test harness
    pub fn new(adaptor: A, tests: Vec<R>) -> Self {
        Self {
            adaptor,
            tests,
        }
    }

    fn translate_assertion(&self, _assertion: TestAssert) -> Feedback {
        // TODO
        Feedback::Success
    }

    fn translate_ui_op(&self, _op: TestOp) -> Feedback {
        // TODO
        Feedback::Success
    }

    fn translate_instruction(&self, instruction: Instruction) -> Feedback {
        match instruction {
            Instruction::Assertion(a) => self.translate_assertion(a),
            Instruction::Operation(i) => self.translate_ui_op(i),
        }
    }

    /// Perform the tests
    pub fn execute(mut self) -> Result<A, Vec<TestMetadata>> {
        // TODO
        let tests: Vec<R> = self.tests.drain(..).collect();
        let mut failures = Vec::with_capacity(tests.len());
        for mut test in tests {
            let mut feedback = Feedback::Start;
            let mut is_success = true;
            let metadata = test.meta();
            log::info!("Starting test {}: {}", metadata.id, metadata.name);
            while let Some(instruction) = test.next(feedback) {
                feedback = self.translate_instruction(instruction);
                is_success &= feedback.is_ok();
            }
            let mut metadata = test.meta();
            metadata.success &= is_success;
            if metadata.success {
                log::info!("{}", metadata);
            } else {
                log::error!("{}", metadata);
                failures.push(metadata);
            }
        }
        if failures.is_empty() {
            Ok(self.adaptor)
        } else {
            Err(failures)
        }

    }
}
