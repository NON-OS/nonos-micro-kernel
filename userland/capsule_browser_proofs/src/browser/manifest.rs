// NONOS Operating System (AGPL-3.0-or-later)
//! Viewport constants the cascade and layout read from the capsule manifest.
//! The real manifest builds an app_skeleton descriptor that needs a running
//! window server, so the proofs carry the two values the engine actually
//! reads and nothing else.
pub const WIDTH: u32 = 1360;
pub const HEIGHT: u32 = 760;
