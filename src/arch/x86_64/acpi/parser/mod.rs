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

pub mod fadt;
mod getters_core;
mod getters_table;
pub mod init;
pub mod madt;
pub mod other;
pub mod phys;
mod root_rsdt;
mod root_xsdt;
pub mod rsdp;
pub mod state;

pub use getters_core::{
    has_8042, has_legacy_pics, hpet_address, interrupt_overrides, ioapics, lapic_address,
    nmi_configs, numa_regions, oem_id, pcie_segments, pm_profile, processors, revision,
    sci_interrupt,
};
pub use getters_table::{has_table, stats, table_address, with_data};
pub use init::init;
pub use root_rsdt::parse_rsdt;
pub use root_xsdt::parse_xsdt;
pub use rsdp::set_rsdp_address;
pub use state::is_initialized;
