/// Test runner invoked by the test harness.
/// A lot like std::iter::Iterator but which accepts input information.
pub trait TestRunner: Send + Sync {
    /// Perform next action
    fn next(&mut self, feedback: super::Feedback) -> Option<super::Instruction>;

    /// Get test information
    fn meta(&self) -> TestMetadata;
}

/// Information about the test and the run
#[derive(Default, Clone, Debug)]
pub struct TestMetadata {
    /// Test name
    pub name: String,

    /// Test ID
    pub id: String,

    /// Test dump file
    pub output: Option<std::path::PathBuf>,

    /// Test author
    pub author: Option<String>,

    /// Was the test successful, or (if incomplete) is it currently passing?
    pub success: bool,
}

impl std::fmt::Display for TestMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEST {}: {}", self.id, self.name)?;
        if let Some(author) = &self.author {
            write!(f, " by {}", author)?;
        }
        if self.success {
            write!(f, " SUCCESS")?;
        } else {
            write!(f, " FAILURE")?;
        }
        if let Some(output) = &self.output {
            write!(f, " ({})", output.display())?;
        }
        Ok(())
    }
}
