// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the RTL8821CE power-sequence engine: the executor that every
//! Realtek MAC power transition runs on. The real `regs` and `pwr` source are
//! included and driven against a modeled register file so the exact
//! read-modify-write, poll and ordering behaviour is checked without hardware.

extern crate alloc;

#[path = "../../capsule_driver_rtl8821ce/src/pwr/mod.rs"]
pub mod pwr;
#[path = "../../capsule_driver_rtl8821ce/src/regs.rs"]
pub mod regs;

#[cfg(test)]
mod ddma_tests;
#[cfg(test)]
mod download_tests;
#[path = "../../capsule_driver_rtl8821ce/src/efuse.rs"]
pub mod efuse;
#[cfg(test)]
mod efuse_tests;
#[path = "../../capsule_driver_rtl8821ce/src/fw/mod.rs"]
pub mod fw;
#[cfg(test)]
mod fw_tests;
#[path = "../../capsule_driver_rtl8821ce/src/h2c/mod.rs"]
pub mod h2c;
#[cfg(test)]
mod h2c_tests;
#[path = "../../capsule_driver_rtl8821ce/src/link.rs"]
pub mod link;
#[cfg(test)]
mod link_tests;
#[cfg(test)]
mod linkport_tests;
#[path = "../../capsule_driver_rtl8821ce/src/assoc.rs"]
pub mod assoc;
#[cfg(test)]
mod assoc_tests;
#[path = "../../capsule_driver_rtl8821ce/src/mac/mod.rs"]
pub mod mac;
#[cfg(test)]
mod mac_tests;
#[cfg(test)]
mod mac_trx_tests;
#[path = "../../capsule_driver_rtl8821ce/src/phy/mod.rs"]
pub mod phy;
#[cfg(test)]
mod phy_tests;
#[cfg(test)]
mod phy_rxpath_tests;
#[cfg(test)]
mod prep_tests;
#[cfg(test)]
mod pwr_tests;
#[path = "../../capsule_driver_rtl8821ce/src/ring/mod.rs"]
pub mod ring;
#[path = "../../capsule_driver_rtl8821ce/src/rx/mod.rs"]
pub mod rx;
#[cfg(test)]
mod rx_tests;
#[path = "../../capsule_driver_rtl8821ce/src/scan.rs"]
pub mod scan;
#[cfg(test)]
mod scan_tests;
#[path = "../../capsule_driver_rtl8821ce/src/sec.rs"]
pub mod sec;
#[cfg(test)]
mod sec_tests;
#[cfg(test)]
mod sections_tests;
#[cfg(test)]
mod staging_tests;
#[cfg(test)]
mod tables_tests;
#[path = "../../capsule_driver_rtl8821ce/src/tx/mod.rs"]
pub mod tx;
#[cfg(test)]
mod tx_tests;
