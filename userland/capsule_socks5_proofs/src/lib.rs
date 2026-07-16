// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the SOCKS5 proxy. Includes the real capsule source via
//! `#[path]` and drives it against hand-built bytes.

#[path = "../../capsule_socks5/src/wire/mod.rs"]
pub mod wire;
#[path = "../../capsule_socks5/src/conn.rs"]
pub mod conn;
#[path = "../../capsule_socks5/src/tunnel.rs"]
pub mod tunnel;
#[path = "../../capsule_socks5/src/manager.rs"]
pub mod manager;

#[cfg(test)]
mod wire_tests;
#[cfg(test)]
mod conn_tests;
#[cfg(test)]
mod tunnel_tests;
#[cfg(test)]
mod manager_tests;
