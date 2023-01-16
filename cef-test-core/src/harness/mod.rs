//! Test execution functionality

mod adaptor;
mod feedback;
#[allow(clippy::module_inception)]
mod harness;
mod headless_adaptor;
mod instructions;
mod json_runner;
mod runner;

pub use adaptor::TestAdaptor;
pub use feedback::Feedback;
pub use harness::TestHarness;
pub use headless_adaptor::HeadlessAdaptor;
pub use instructions::{Instruction, TestAssert, GeneralAssertType, ElementAssert, ElementAssertionType, TestOp, ElementSelector, TabSelector, GeneralOpType, BasicOpType, ElementOp, ElementOpType};
pub use json_runner::JsonRunner;
pub use runner::{TestRunner, TestMetadata};
