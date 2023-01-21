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
    /// Tab-related assertion
    Tab(TabAssert),
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
    /// Assert element text value
    Value(Comparison),
    /// Assert element attribute
    Attribute {
        /// Attribute name
        attribute: String,
        /// Assertion comparison mode
        comparison: Comparison
    },

}

/// Assertion operations
pub enum TabAssert {
    /// Run javascript and validate the result
    Evaluate {
        /// Javascript to execute
        script: String,
        /// Assertion comparison mode
        comparison: Comparison,
    }
}

/// Assertion compare operation to perform
pub enum Comparison {
    /// Assert non-null
    Exists,
    /// Assert not empty
    ExistsNotEmpty,
    /// Assert equals expected text
    TextEquals(String),
    /// Assert contains expected string
    TextContains(String),
    /// Assert == expected value
    Equals(serde_json::Value),
    /// Assert actual != expected value
    NotEquals(serde_json::Value),
    /*
    /// Assert actual < expected value
    LessThan(serde_json::Value),
    /// Assert actual <= expected value
    LessThanEquals(serde_json::Value),
    /// Assert actual > expected value
    GreaterThan(serde_json::Value),
    /// Assert actual >= expected value
    GreaterThanEquals(serde_json::Value),
    */
}

impl Comparison {
    /// Compare actual value
    pub fn compare(&self, value: Option<&serde_json::Value>) -> bool {
        match self {
            Self::Exists => !value.is_none(),
            Self::ExistsNotEmpty => {
                if let Some(value) = value {
                    if let Some(s) = value.as_str() {
                        !s.is_empty()
                    } else {
                        !value.is_null()
                    }
                } else {
                    false
                }
            },
            _ => {
                let value = value.unwrap_or(&serde_json::Value::Null);
                match self {
                    Self::TextEquals(expected) => {
                        if let Some(actual) = value.as_str() {
                            actual.trim() == expected
                        } else {
                            false
                        }
                    },
                    Self::TextContains(expected) => {
                        if let Some(actual) = value.as_str() {
                            actual.contains(expected)
                        } else {
                            false
                        }
                    },
                    Self::Equals(expected) => value == expected,
                    Self::NotEquals(expected) => value != expected,

                    Self::Exists => unreachable!(),
                    Self::ExistsNotEmpty => unreachable!(),
                }
            }
        }
    }

    /// Display-friendly representation of the assertion with actual and expected values
    pub fn pseudocode_assert(&self, value: Option<&serde_json::Value>) -> String {
        match value {
            Some(value) => {
                match self {
                    Self::Exists => format!("{} must exist", value),
                    Self::ExistsNotEmpty => format!("{} must exist", value),
                    Self::TextEquals(expected) => format!("\"{}\" must equal \"{}\"", value, expected),
                    Self::TextContains(expected) => format!("\"{}\" must contain \"{}\"", value, expected),
                    Self::Equals(expected) => format!("{} == {}", value, expected),
                    Self::NotEquals(expected) => format!("{} != {}", value, expected),
                }
            },
            None => {
                let value = serde_json::Value::Null;
                match self {
                    Self::Exists => format!("None must exist (contradiction!)"),
                    Self::ExistsNotEmpty => format!("None must exist (contradiction!)"),
                    Self::TextEquals(expected) => format!("\"{}\" must equal \"{}\"", value, expected),
                    Self::TextContains(expected) => format!("\"{}\" must contain \"{}\"", value, expected),
                    Self::Equals(expected) => format!("{} == {}", value, expected),
                    Self::NotEquals(expected) => format!("{} != {}", value, expected),
                }
            }
        }

    }
}

/// User interface interaction
pub struct TestOp {
    /// Tab context
    pub context: TabSelector,
    /// Test operation
    pub op: GeneralOpType,
}

/// Element selection mode
#[allow(clippy::upper_case_acronyms)]
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
    /// Select by tab title regex pattern
    TitleRegex(String),
    /// Select by tab's current URL
    Url(String),
    /// Select by tab's current URL regex pattern
    UrlRegex(String),
    /// Select by tab identifier
    Id(String),
}

impl std::fmt::Display for TabSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Title(title) => write!(f, "Tab[title==`{}`]", title),
            Self::TitleRegex(title) => write!(f, "Tab[title~=`{}`]", title),
            Self::Url(url) => write!(f, "Tab[url==`{}`]", url),
            Self::UrlRegex(url) => write!(f, "Tab[url~=`{}`]", url),
            Self::Id(id) => write!(f, "Tab[id==`{}`]", id),
        }
    }
}

/// Test operation information
pub enum GeneralOpType {
    /// Operate on an element
    Element(ElementOp),
    /// Tab context operation
    Tab(TabOpType),
}

/// Basic operation type
pub enum TabOpType {
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

