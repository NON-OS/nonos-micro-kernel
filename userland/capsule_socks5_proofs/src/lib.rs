// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the SOCKS5 proxy. Includes the real capsule source via
//! `#[path]` and drives it against hand-built bytes.

#[path = "../../capsule_socks5/src/conn/mod.rs"]
pub mod conn;
#[path = "../../capsule_socks5/src/server/inbox.rs"]
pub mod inbox;
#[path = "../../capsule_socks5/src/manager/mod.rs"]
pub mod manager;
#[path = "../../capsule_socks5/src/server/reply.rs"]
pub mod reply;
#[path = "../../capsule_socks5/src/server/request.rs"]
pub mod request;
#[path = "../../capsule_socks5/src/tunnel/mod.rs"]
pub mod tunnel;
#[path = "../../capsule_socks5/src/nym/watch.rs"]
pub mod watch;
#[path = "../../capsule_socks5/src/wire/mod.rs"]
pub mod wire;

#[cfg(test)]
mod conn_tests;
#[cfg(test)]
mod inbox_tests;
#[cfg(test)]
mod manager_tests;
#[cfg(test)]
mod reply_tests;
#[cfg(test)]
mod tunnel_tests;
#[cfg(test)]
mod watch_tests;
#[cfg(test)]
mod wire_tests;
