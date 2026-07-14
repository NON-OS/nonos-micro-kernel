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

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::RwLock;

use crate::arch::x86_64::acpi::data::{AcpiData, AcpiStats};

pub static INITIALIZED: AtomicBool = AtomicBool::new(false);
pub static TABLES: RwLock<Option<TableRegistry>> = RwLock::new(None);
pub static STATS: RwLock<AcpiStats> = RwLock::new(AcpiStats::new());

pub struct TableRegistry {
    pub tables: BTreeMap<u32, u64>,
    // Every SSDT physical address, kept in full: SSDTs share one signature so
    // the signature-keyed map above collapses them to a single entry, yet a
    // touchpad (or any device) is frequently described in a secondary SSDT.
    pub ssdt_addresses: Vec<u64>,
    pub data: AcpiData,
}

impl TableRegistry {
    pub fn new() -> Self {
        Self { tables: BTreeMap::new(), ssdt_addresses: Vec::new(), data: AcpiData::new() }
    }
}

pub fn is_initialized() -> bool {
    INITIALIZED.load(Ordering::Acquire)
}
