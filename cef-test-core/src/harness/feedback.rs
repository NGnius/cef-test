/// Harness information for a test runner
pub enum Feedback {
    /// Start of run (no feedback to provide)
    Start,
    /// Last instruction was successful
    Success,
    /// Last instruction was an assertion and it failed
    AssertFailure,
    /// Last instruction raised an error
    Error,
}

impl Feedback {
    /// Feedback is indicative of regular operations
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Success => true,
            Self::Start => true,
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
}
