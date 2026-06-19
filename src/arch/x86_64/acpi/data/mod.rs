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

mod acpi_data_methods;
mod acpi_data_new;
mod acpi_data_struct;
mod interrupt;
mod ioapic;
mod numa;
mod pcie;
mod processor;
mod stats;

pub use acpi_data_struct::AcpiData;
pub use interrupt::{InterruptOverride, NmiConfig};
pub use ioapic::IoApicInfo;
pub use numa::NumaMemoryRegion;
pub use pcie::PcieSegment;
pub use processor::ProcessorInfo;
pub use stats::AcpiStats;
