use super::{TestRunner, TestAdapter, TestMetadata};
use super::{Instruction, TestAssert, TestOp, Feedback, GeneralOpType, ElementOpType, TabOpType, GeneralAssertType, ElementAssertionType, TabAssert, Comparison};

/// Harness which runs one or more tests
pub struct TestHarness<R: TestRunner, A: TestAdapter> {
    tests: Vec<R>,
    adapter: A,
}

impl<R: TestRunner, A: TestAdapter> TestHarness<R, A> {
    /// Construct a new test harness
    pub fn new(adapter: A, tests: Vec<R>) -> Self {
        Self {
            adapter,
            tests,
        }
    }

    fn translate_assertion(&mut self, assertion: TestAssert) -> Feedback {
        match assertion.assertion {
            GeneralAssertType::Element(elem) => {
                match elem.assert {
                    ElementAssertionType::Value(comparison) =>
                        Self::maybe_assert(
                            self.adapter.element_value(&assertion.context, &elem.element),
                            comparison
                        ),
                    ElementAssertionType::Attribute { attribute, comparison } =>
                        Self::maybe_assert(
                            self.adapter.element_attribute(&assertion.context, &elem.element, &attribute),
                            comparison
                        )
                }
            },
            GeneralAssertType::Tab(TabAssert::Evaluate { script, comparison }) => Self::maybe_assert(self.adapter.evaluate(&assertion.context, &script), comparison)
        }
    }

    fn maybe_assert(adaptor_feedback: Feedback, cmp: Comparison) -> Feedback {
        if let Feedback::Value(v) = &adaptor_feedback {
            if cmp.compare(Some(v)) {
                log::info!("Assertion satisfied: {}", cmp.pseudocode_assert(Some(v)));
                adaptor_feedback
            } else {
                log::error!("Assertion failed: {}", cmp.pseudocode_assert(Some(v)));
                Feedback::AssertFailure
            }
        } else {
            if cmp.compare(None) {
                log::info!("Assertion satisfied: {}", cmp.pseudocode_assert(None));
                adaptor_feedback
            } else {
                log::error!("Assertion failed: {}", cmp.pseudocode_assert(None));
                Feedback::AssertFailure
            }
        }
    }

    fn translate_ui_op(&mut self, op: TestOp) -> Feedback {
        match op.op {
            GeneralOpType::Element(elem) => {
                match elem.op {
                    ElementOpType::Click => self.adapter.element_click(&op.context, &elem.context),
                    ElementOpType::WaitFor => self.adapter.element_wait(&op.context, &elem.context),
                    ElementOpType::Focus => self.adapter.element_focus(&op.context, &elem.context),
                    ElementOpType::ScrollTo => self.adapter.element_scroll_to(&op.context, &elem.context),
                }
            },
            GeneralOpType::Tab(TabOpType::Sleep(ms)) => self.adapter.wait(&op.context, ms),
            GeneralOpType::Tab(TabOpType::Evaluate(js)) => self.adapter.evaluate(&op.context, &js),
        }
    }

    fn translate_instruction(&mut self, instruction: Instruction) -> Feedback {
        match instruction {
            Instruction::Assertion(a) => self.translate_assertion(a),
            Instruction::Operation(i) => self.translate_ui_op(i),
        }
    }

    /// Perform the tests
    pub fn execute(mut self) -> Result<A, Vec<TestMetadata>> {
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
            Ok(self.adapter)
        } else {
            Err(failures)
        }

    }
}
