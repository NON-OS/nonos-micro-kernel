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

use core::sync::atomic::Ordering;

use super::fadt::parse_fadt;
use super::madt::parse_madt;
use super::other::{parse_dmar, parse_hpet, parse_mcfg, parse_srat};
use super::rsdp::find_rsdp;
use super::state::{TableRegistry, INITIALIZED, STATS, TABLES};
use super::{parse_rsdt, parse_xsdt};
use crate::arch::x86_64::acpi::error::{AcpiError, AcpiResult};

pub fn init() -> AcpiResult<()> {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Err(AcpiError::AlreadyInitialized);
    }

    let rsdp = find_rsdp()?;

    let mut registry = TableRegistry::new();

    registry.data.revision = rsdp.base.revision;
    registry.data.oem_id = rsdp.base.oem_id;

    if rsdp.has_xsdt() {
        parse_xsdt(&mut registry, rsdp.xsdt_address)?;
    } else if rsdp.rsdt_address() != 0 {
        parse_rsdt(&mut registry, rsdp.rsdt_address() as u64)?;
    } else {
        INITIALIZED.store(false, Ordering::SeqCst);
        return Err(AcpiError::NoRootTable);
    }

    parse_fadt(&mut registry)?;
    parse_madt(&mut registry);
    parse_hpet(&mut registry);
    parse_mcfg(&mut registry);
    parse_srat(&mut registry);
    parse_dmar(&mut registry);

    {
        let mut stats = STATS.write();
        stats.tables_found = registry.tables.len() as u32;
        stats.processors_found = registry.data.processors.len() as u32;
        stats.ioapics_found = registry.data.ioapics.len() as u32;
        stats.overrides_found = registry.data.overrides.len() as u32;
        stats.pcie_segments = registry.data.pcie_segments.len() as u32;

        let mut nodes: alloc::collections::BTreeSet<u32> = alloc::collections::BTreeSet::new();
        for region in &registry.data.numa_regions {
            nodes.insert(region.proximity_domain);
        }
        stats.numa_nodes = nodes.len() as u32;
    }

    *TABLES.write() = Some(registry);

    Ok(())
}
