// NONOS Operating System (AGPL-3.0-or-later)
// The real host-command queue pieces from the iwlwifi driver. `ring`, `tfd`,
// `doorbell` and `send` resolve their `crate::constants` / `crate::regs`
// imports against the modules this proof crate already includes.
#[path = "../../../capsule_driver_iwlwifi/src/hcmd/header.rs"]
pub mod header;
#[path = "../../../capsule_driver_iwlwifi/src/hcmd/ring.rs"]
pub mod ring;
#[path = "../../../capsule_driver_iwlwifi/src/hcmd/tfd.rs"]
pub mod tfd;
#[path = "../../../capsule_driver_iwlwifi/src/hcmd/doorbell.rs"]
pub mod doorbell;
#[path = "../../../capsule_driver_iwlwifi/src/hcmd/send.rs"]
pub mod send;
