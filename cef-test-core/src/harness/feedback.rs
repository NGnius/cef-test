/// Harness information for a test runner
#[derive(Clone)]
pub enum Feedback {
    /// Start of run (no feedback to provide)
    Start,
    /// Last instruction was successful
    Success,
    /// Last instruction returned a value
    Value(serde_json::Value),
    /// Last instruction was an assertion and it failed
    AssertFailure,
    /// Last instruction raised an error
    Error,
    /// Last instruction was not supported by adaptor
    Unsupported,
}

impl Feedback {
    /// Feedback is indicative of regular operations
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Success => true,
            Self::Start => true,
            Self::Value(_) => true,
            _ => false,
        }
    }

    /// Feedback is indicative of an error
    pub fn is_err(&self) -> bool {
        match self {
            Self::Error => true,
            _ => false,
        }
    }

    /// Feedback is indicative of a failing test
    pub fn is_fail(&self) -> bool {
        match self {
            Self::AssertFailure => true,
            Self::Error => true,
            Self::Unsupported => true,
            _ => false,
        }
    }
}
