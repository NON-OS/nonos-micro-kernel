//! The reply block serialisation, pulled in by path. The builder and the key
//! store need the whole capsule around them, so only what can stand alone is
//! taken here.

#[path = "../../../src/surb/types.rs"]
pub mod types;

#[path = "../../../src/surb/bytes.rs"]
pub mod bytes;

pub use bytes::surb_bytes;
pub use types::{ReplySurb, SURB_KEY_BYTES};

// The builder needs the key store and the topology, which do not stand alone,
// so it is taken with the shims the harness already provides.
#[path = "../../../src/surb/build.rs"]
pub mod build;

pub use build::build_surb;
