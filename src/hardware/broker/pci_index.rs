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

//! Kernel-private side table mapping a broker `device_id` back to
//! the PCI bits the kernel needs in order to program MSI/MSI-X on
//! behalf of a capsule. The wire-form `DeviceRecord` deliberately
//! does not carry MSI capability descriptors or the BDF — those
//! are kernel-only and must never reach a capsule. This table is
//! how the broker bridges between the opaque id the capsule sees
//! and the structures the PCI layer requires.

extern crate alloc;

use alloc::vec::Vec;
use spin::RwLock;

use crate::drivers::pci::types::{MsixInfo, PciAddress, PciBar};

#[derive(Clone, Copy, Debug)]
pub(in crate::hardware::broker) struct PciHandle {
    pub(in crate::hardware::broker) device_id: u64,
    pub(in crate::hardware::broker) address: PciAddress,
    pub(in crate::hardware::broker) bars: [PciBar; 6],
    pub(in crate::hardware::broker) msix: Option<MsixInfo>,
}

static INDEX: RwLock<Vec<PciHandle>> = RwLock::new(Vec::new());

pub(in crate::hardware::broker) fn install(handles: Vec<PciHandle>) {
    *INDEX.write() = handles;
}

pub(in crate::hardware::broker) fn lookup(device_id: u64) -> Option<PciHandle> {
    INDEX.read().iter().find(|h| h.device_id == device_id).copied()
}
