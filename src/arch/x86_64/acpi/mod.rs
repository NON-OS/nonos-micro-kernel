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

pub mod aml;
mod api;
pub mod data;
pub mod devices;
pub mod error;
mod error_strings;
mod error_types;
pub mod parser;
pub mod power;
mod power_query;
pub mod power_reboot;
mod power_sleep;
mod power_types;
pub mod tables;

pub use api::madt;
pub use api::{
    has_legacy_pics, has_table, hpet_address, init, interrupt_overrides, ioapics, is_initialized,
    lapic_address, nmi_configs, numa_regions, oem_id, pcie_segments, pm_profile, processors,
    revision, sci_interrupt, set_rsdp_address, stats, table_address,
};
pub use data::{
    AcpiData, AcpiStats, InterruptOverride, IoApicInfo, NmiConfig, NumaMemoryRegion, PcieSegment,
    ProcessorInfo,
};
pub use devices::PciDevice;
pub use error::{AcpiError, AcpiResult};
pub use power::SleepState;
pub use tables::{
    fadt_flags, madt_flags, AddressSpace, Fadt, GenericAddress, Hpet, Madt, MadtEntryHeader,
    MadtEntryType, MadtInterruptOverride, MadtIoApic, MadtLocalApic, MadtLocalApicNmi,
    MadtLocalApicOverride, MadtLocalX2Apic, MadtLocalX2ApicNmi, MadtNmiSource, Mcfg, McfgEntry,
    PmProfile, Rsdp, RsdpExtended, SdtHeader, Slit, Srat, SratEntryType, SratMemoryAffinity,
    SratProcessorAffinity, SratX2ApicAffinity, RSDP_ALIGNMENT, RSDP_SIGNATURE, SIG_FADT, SIG_HPET,
    SIG_MADT, SIG_MCFG, SIG_RSDT, SIG_SLIT, SIG_SRAT, SIG_XSDT,
};
