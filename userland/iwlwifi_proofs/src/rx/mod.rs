// NONOS Operating System (AGPL-3.0-or-later)
// The real receive path from the iwlwifi driver. `recv` resolves its
// `crate::constants` / `crate::rx` imports against the modules this proof crate
// includes.
#[path = "../../../capsule_driver_iwlwifi/src/rx/packet.rs"]
pub mod packet;
#[path = "../../../capsule_driver_iwlwifi/src/rx/ring.rs"]
pub mod ring;
#[path = "../../../capsule_driver_iwlwifi/src/rx/recv.rs"]
pub mod recv;
