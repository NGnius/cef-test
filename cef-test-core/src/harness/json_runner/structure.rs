use serde::{Deserialize, Serialize};

use super::super::TestMetadata;

/// Test descriptor
#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub(super) info: TestInfo,
    pub(super) test: Vec<TestInstruction>,
}

/// Test metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestInfo {
    pub name: String,
    pub blame: String,
    pub id: String,
    pub output: String,
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

/// Test metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct TestInstruction {}
