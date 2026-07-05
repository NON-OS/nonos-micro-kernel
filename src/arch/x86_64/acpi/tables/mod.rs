// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

mod dmar_types;
pub mod fadt;
pub mod hpet;
pub mod madt;
mod mcfg_bridge;
mod mcfg_class;
mod mcfg_config;
mod mcfg_header;
mod mcfg_types;
pub mod rsdp;
mod rsdp_base;
mod rsdp_extended;
pub mod sdt;
pub mod slit;
pub mod srat;
mod srat_memory;
mod srat_other;
mod srat_processor;
mod srat_types;

pub use dmar_types::*;
pub use fadt::*;
pub use hpet::*;
pub use madt::*;
pub use mcfg_bridge::*;
pub use mcfg_class::*;
pub use mcfg_config::*;
pub use mcfg_header::*;
pub use mcfg_types::*;
pub use rsdp_base::{
    Rsdp, BIOS_ROM_SIZE, BIOS_ROM_START, EBDA_PTR_ADDR, RSDP_ALIGNMENT, RSDP_SIGNATURE,
};
pub use rsdp_extended::RsdpExtended;
pub use sdt::*;
pub use slit::*;
pub use srat_memory::*;
pub use srat_other::*;
pub use srat_processor::*;
pub use srat_types::*;

pub const SIG_RSDT: u32 = u32::from_le_bytes(*b"RSDT");
pub const SIG_XSDT: u32 = u32::from_le_bytes(*b"XSDT");
pub const SIG_FADT: u32 = u32::from_le_bytes(*b"FACP");
pub const SIG_MADT: u32 = u32::from_le_bytes(*b"APIC");
pub const SIG_HPET: u32 = u32::from_le_bytes(*b"HPET");
pub const SIG_MCFG: u32 = u32::from_le_bytes(*b"MCFG");
pub const SIG_DMAR: u32 = u32::from_le_bytes(*b"DMAR");
pub const SIG_SRAT: u32 = u32::from_le_bytes(*b"SRAT");
pub const SIG_SLIT: u32 = u32::from_le_bytes(*b"SLIT");
pub const SIG_DSDT: u32 = u32::from_le_bytes(*b"DSDT");
pub const SIG_SSDT: u32 = u32::from_le_bytes(*b"SSDT");
