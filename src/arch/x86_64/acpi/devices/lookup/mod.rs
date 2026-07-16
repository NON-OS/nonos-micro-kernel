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

//! ACPI device lookup accessors.

mod enabled_processor_count;
mod get_hpet_base;
mod get_ioapic_addresses;
mod get_ioapic_for_gsi;
mod get_lapic_base;
mod get_pcie_ecam;
mod has_8042;
mod has_legacy_pics;
mod numa_domains;
mod processor_count;

pub use enabled_processor_count::enabled_processor_count;
pub use get_hpet_base::get_hpet_base;
pub use get_ioapic_addresses::get_ioapic_addresses;
pub use get_ioapic_for_gsi::get_ioapic_for_gsi;
pub use get_lapic_base::get_lapic_base;
pub use get_pcie_ecam::get_pcie_ecam;
pub use has_8042::has_8042;
pub use has_legacy_pics::has_legacy_pics;
pub use numa_domains::numa_domains;
pub use processor_count::processor_count;
