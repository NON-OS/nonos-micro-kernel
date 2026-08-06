//! Reply reassembly and the message layer above it, pulled in by path. The
//! opener needs the capsule's key store around it, so only what stands alone
//! is taken here.

#[path = "../../../src/reply/assembly.rs"]
pub mod assembly;

#[path = "../../../src/reply/reassemble.rs"]
pub mod reassemble;

#[path = "../../../src/reply/types.rs"]
pub mod types;

#[path = "../../../src/reply/message.rs"]
pub mod message;

pub use message::reply_body;
pub use assembly::Assembly;
pub use reassemble::collect;
pub use types::{TAG_REPLY_DATA, TYPE_REPLY};
