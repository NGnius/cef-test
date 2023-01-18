//! Test execution functionality

mod adapter;
mod feedback;
#[allow(clippy::module_inception)]
mod harness;
mod headless_adapter;
mod instructions;
mod json_runner;
mod runner;

pub use adapter::TestAdapter;
pub use feedback::Feedback;
pub use harness::TestHarness;
pub use headless_adapter::HeadlessAdapter;
pub use instructions::{Instruction, TestAssert, GeneralAssertType, ElementAssert, ElementAssertionType, TestOp, ElementSelector, TabSelector, GeneralOpType, BasicOpType, ElementOp, ElementOpType};
pub use json_runner::JsonRunner;
pub use runner::{TestRunner, TestMetadata};
