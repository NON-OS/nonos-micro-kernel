//! The node record the API reader fills in, pulled in by path so the test
//! sees the shipping definition. Only the types are needed here: the store,
//! the clock and the route selector belong to the running capsule.

#[path = "../../../src/topology/types.rs"]
pub mod types;

pub use types::{Node, Role};
