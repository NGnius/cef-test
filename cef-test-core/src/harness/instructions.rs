/// Instruction for the text harness to perform
pub enum Instruction {
    /// Test assertion
    Assertion(TestAssert),
    /// Test harness operation
    Operation(TestOp),
}

/// Assertion
pub struct TestAssert {
    /// Tab context
    pub context: TabSelector,
    /// Test assertion
    pub assertion: GeneralAssertType,
}

/// Test assertion information
pub enum GeneralAssertType {
    /// Element-related assertion
    Element(ElementAssert),
}

/// Element assertion
pub struct ElementAssert {
    /// Element to target
    pub element: ElementSelector,
    /// Test assertion
    pub assert: ElementAssertionType,
}

/// Assertion operations
pub enum ElementAssertionType {
    /// Assert element exists
    Exists,
    /// Assert element contains text
    TextEquals(String)
}

/// User interface interaction
pub struct TestOp {
    /// Tab context
    pub context: TabSelector,
    /// Test operation
    pub op: GeneralOpType,
}

/// Element selection mode
pub enum ElementSelector {
    /// Use CSS selector syntax
    CSS(String),
}

impl std::fmt::Display for ElementSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CSS(pattern) => write!(f, "Element[css~`{}`]", pattern),
        }
    }
}

/// Tab selection mode
pub enum TabSelector {
    /// Select by tab title
    Title(String),
    /// Select by tab's current URL
    Url(String),
    /// Select by tab identifier
    Id(String),
}

impl std::fmt::Display for TabSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Title(title) => write!(f, "Tab[title==`{}`]", title),
            Self::Url(url) => write!(f, "Tab[url==`{}`]", url),
            Self::Id(id) => write!(f, "Tab[id==`{}`]", id),
        }
    }
}

/// Test operation information
pub enum GeneralOpType {
    /// Operate on an element
    Element(ElementOp),
    /// Basic context operation
    Basic(BasicOpType),
}

/// Basic operation type
pub enum BasicOpType {
    /// Pause executing thread for time, in milliseconds
    Sleep(u64),
    /// Execute Javascript in the global tab context
    Evaluate(String),
}

/// Element manipulation operation
pub struct ElementOp {
    /// Element to target
    pub context: ElementSelector,
    /// Operation to perform
    pub op: ElementOpType,
}

/// Element operation type
pub enum ElementOpType {
    /// Click on element
    Click,
    /// Wait for element to be created
    WaitFor,
    /// Focus the element
    Focus,
    /// Scroll the element into view
    ScrollTo,
}

