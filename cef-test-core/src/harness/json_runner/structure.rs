use std::convert::From;
use serde::{Deserialize, Serialize};

use super::super::TestMetadata;
use super::super::{TabSelector, ElementSelector, ElementOpType, ElementOp, BasicOpType, GeneralOpType, GeneralAssertType, ElementAssert, ElementAssertionType, Instruction, TestAssert, TestOp};

/// Test descriptor
#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub(super) info: TestInfo,
    pub(super) test: Vec<TestStep>,
}

/// Test metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestInfo {
    pub name: String,
    pub blame: String,
    pub id: String,
    pub output: String,
    pub fail_mode: FailureMode,
}

/// Failure behaviour
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FailureMode {
    SkipInstructions,
    SkipSteps,
    FastFail,
}

impl std::convert::From<TestInfo> for TestMetadata {
    fn from(other: TestInfo) -> Self {
        TestMetadata {
            name: other.name,
            id: other.id,
            output: Some(other.output.into()),
            author: Some(other.blame),
            success: true,
        }
    }
}

/// Test step
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestStep {
    pub tab: TabDescriptor,
    pub operations: Vec<TestInstruction>,
}

/// Tab metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "by")]
pub enum TabDescriptor {
    /// Select by tab title
    Title{title: String},
    /// Select by tab's current URL
    Url{url: String},
    /// Select by tab identifier
    Id{id: String},
}

impl From<TabDescriptor> for TabSelector {
    fn from(value: TabDescriptor) -> Self {
        match value {
            TabDescriptor::Title{title: t} => Self::Title(t),
            TabDescriptor::Url{url: u} => Self::Url(u),
            TabDescriptor::Id{id: i} => Self::Id(i),
        }
    }
}

/// Test instruction
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum TestInstruction {
    /// Operate on an element
    Element(TestElementInstruction),
    /// Pause executing thread for time, in milliseconds
    Sleep {
        /// Duration of pause
        milliseconds: u64,
    },
    Eval {
        /// Javascript to execute
        code: String,
    },
    /// Assertion on an element
    Assert(TestAssertionInstruction),
}

impl TestInstruction {
    pub fn into_instruction(self, tab: TabDescriptor) -> Instruction {
        let selector: TabSelector = tab.into();
        match self {
            TestInstruction::Element(elem) => Instruction::Operation(TestOp {
                context: selector,
                op: GeneralOpType::Element(elem.into()),
            }),
            TestInstruction::Sleep { milliseconds } => Instruction::Operation(TestOp {
                context: selector,
                op: GeneralOpType::Basic(BasicOpType::Sleep(milliseconds)),
            }),
            TestInstruction::Eval { code } => Instruction::Operation(TestOp {
                context: selector,
                op: GeneralOpType::Basic(BasicOpType::Evaluate(code)),
            }),
            TestInstruction::Assert(assertion) => Instruction::Assertion(TestAssert {
                context: selector,
                assertion: GeneralAssertType::Element(assertion.into()),
            }),
        }
    }
}

/// Test element instruction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestElementInstruction {
    pub element: ElementDescriptor,
    pub operation: ElementInteraction,
}

impl From<TestElementInstruction> for ElementOp {
    fn from(value: TestElementInstruction) -> Self {
        Self {
            context: value.element.into(),
            op: value.operation.into(),
        }
    }
}

/// Test element instruction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestAssertionInstruction {
    pub element: ElementDescriptor,
    pub assert: ElementAssertion,
}

impl From<TestAssertionInstruction> for ElementAssert {
    fn from(value: TestAssertionInstruction) -> Self {
        Self {
            element: value.element.into(),
            assert: value.assert.into(),
        }
    }
}

/// Element descriptor
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "by")]
pub enum ElementDescriptor {
    /// Use CSS selector syntax
    CSS{css: String},
}

impl From<ElementDescriptor> for ElementSelector {
    fn from(value: ElementDescriptor) -> Self {
        match value {
            ElementDescriptor::CSS{css: s} => Self::CSS(s),
        }
    }
}

/// Element operation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ElementInteraction {
    /// Click on element
    Click,
    /// Wait for element to be created
    WaitFor,
    /// Focus the element
    Focus,
    /// Scroll the element into view
    ScrollTo,
}

impl From<ElementInteraction> for ElementOpType {
    fn from(value: ElementInteraction) -> Self {
        match value {
            ElementInteraction::Click => Self::Click,
            ElementInteraction::WaitFor => Self::WaitFor,
            ElementInteraction::Focus => Self::Focus,
            ElementInteraction::ScrollTo => Self::ScrollTo,
        }
    }
}

/// Element operation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ElementAssertion {
    /// Assert element exists
    Exists,
    /// Assert element contains text
    TextEquals(String)
}

impl From<ElementAssertion> for ElementAssertionType {
    fn from(value: ElementAssertion) -> Self {
        match value {
            ElementAssertion::Exists => Self::Exists,
            ElementAssertion::TextEquals(t) => Self::TextEquals(t),
        }
    }
}
