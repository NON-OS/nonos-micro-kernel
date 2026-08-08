//! Just enough of the capsule's `gateway_client` for the handshake to compile
//! against unchanged sources. The handshake tree below is the real one, pulled
//! in by path rather than copied, so this test cannot drift from what ships.

pub mod ws;

#[path = "../../../../src/gateway_client/handshake/mod.rs"]
pub mod handshake;
