/// Instruction for the text harness to perform
pub enum Instruction {
    /// Test assertion
    Assertion(TestAssert),
    /// UI manipulation
    Interaction(UIOp),
}

/// Assertion
pub enum TestAssert {}

/// User interface interaction
pub enum UIOp {}
