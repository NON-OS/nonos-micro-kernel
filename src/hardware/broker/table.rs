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

extern crate alloc;

use alloc::vec::Vec;
use spin::RwLock;

use crate::drivers::pci::types::{PciBar, PciDevice};

use super::class::{classify_pci, ids as class_ids};
use super::device::{Bar, BarKind, BusKind, DeviceRecord, BAR_FLAG_MEM64, BAR_FLAG_PREFETCH};
use super::pci_index::PciHandle;

static TABLE: RwLock<Vec<DeviceRecord>> = RwLock::new(Vec::new());

// Populate the broker device table from a PCI scan. Called once at
// boot from `core_init` after `bus::pci::init()`. Subsequent calls
// replace the table.
pub fn init_from_pci(devices: &[PciDevice]) {
    let mut records: Vec<DeviceRecord> = Vec::with_capacity(devices.len());
    let mut handles: Vec<PciHandle> = Vec::with_capacity(devices.len());
    for (idx, dev) in devices.iter().enumerate() {
        records.push(record_from_pci(idx as u64, dev));
        handles.push(PciHandle {
            device_id: idx as u64,
            address: dev.address,
            bars: dev.bars,
            msix: dev.msix,
        });
    }
    *TABLE.write() = records;
    super::pci_index::install(handles);
}

// Append a platform/ACPI device record. Used after the PCI scan
// for legacy devices that do not enumerate through PCI (PS/2
// keyboard / mouse, COM ports, etc). The device_id is assigned
// here so it does not collide with the PCI ids used above.
pub fn register_platform_device(mut record: DeviceRecord) -> u64 {
    let mut table = TABLE.write();
    let next_id = table.iter().map(|r| r.device_id).max().map(|m| m + 1).unwrap_or(0x1_0000_0000);
    record.device_id = next_id;
    table.push(record);
    next_id
}

// Read-only snapshot. Returns an owned vec so callers do not hold the
// lock across long operations.
pub fn list() -> Vec<DeviceRecord> {
    TABLE.read().clone()
}

// Filter snapshot by class id. `class == 0` means no filter.
pub fn list_by_class(class: u32) -> Vec<DeviceRecord> {
    if class == 0 {
        return list();
    }
    TABLE.read().iter().filter(|r| r.class == class).copied().collect()
}

// True iff `device_id` is present in the broker table.
pub fn contains(device_id: u64) -> bool {
    TABLE.read().iter().any(|r| r.device_id == device_id)
}

// Look up a single record by id. Returns `None` when the device is
// not in the table; the broker uses this on syscall-rate paths so
// it does not have to clone the whole table to read one field.
pub fn lookup(device_id: u64) -> Option<DeviceRecord> {
    TABLE.read().iter().find(|r| r.device_id == device_id).copied()
}

// Single-field lookup for the device class. Returns `None` when the
// device is not in the table.
pub fn class_of(device_id: u64) -> Option<u32> {
    TABLE.read().iter().find(|r| r.device_id == device_id).map(|r| r.class)
}

fn record_from_pci(device_id: u64, dev: &PciDevice) -> DeviceRecord {
    let mut bars = [Bar::empty(); 6];
    let mut bar_span = 0u8;
    for (i, b) in dev.bars.iter().enumerate() {
        if !b.is_present() {
            continue;
        }
        bars[i] = bar_from_pci(b);
        bar_span = (i + 1) as u8;
    }
    let class = classify_pci(dev.class, dev.subclass, dev.progif).id();
    DeviceRecord {
        device_id,
        bus_kind: BusKind::Pci as u8,
        _pad0: [0; 3],
        class,
        vendor: dev.vendor_id,
        device: dev.device_id,
        flags: 0,
        bar_count: bar_span,
        irq_line: dev.interrupt_line,
        irq_pin: dev.interrupt_pin,
        _pad1: [0; 1],
        irq_source: dev.interrupt_line as u32,
        bars,
        virtio_present: dev.virtio.map_or(0, |_| 1),
        virtio_common_bar: dev.virtio.map_or(0, |v| v.common_bar),
        virtio_notify_bar: dev.virtio.map_or(0, |v| v.notify_bar),
        virtio_device_bar: dev.virtio.map_or(0, |v| v.device_bar),
        virtio_common_off: dev.virtio.map_or(0, |v| v.common_off),
        virtio_notify_off: dev.virtio.map_or(0, |v| v.notify_off),
        virtio_device_off: dev.virtio.map_or(0, |v| v.device_off),
        virtio_isr_off: dev.virtio.map_or(0, |v| v.isr_off),
        virtio_notify_mult: dev.virtio.map_or(0, |v| v.notify_mult),
    }
}

fn bar_from_pci(b: &PciBar) -> Bar {
    if b.is_memory() {
        let mut flags = 0u8;
        if b.is_prefetchable() {
            flags |= BAR_FLAG_PREFETCH;
        }
        if b.is_64bit() {
            flags |= BAR_FLAG_MEM64;
        }
        let base = b.address().map(|p| p.as_u64()).unwrap_or(0);
        Bar { base, size: b.size(), kind: BarKind::Mmio as u8, flags, _pad: [0; 6] }
    } else if b.is_io() {
        let port = b.port().unwrap_or(0) as u64;
        Bar { base: port, size: b.size(), kind: BarKind::Pio as u8, flags: 0, _pad: [0; 6] }
    } else {
        Bar::empty()
    }
}

// Used by tests. The non-test surface is read-only after `init_from_pci`.
#[cfg(test)]
pub(crate) fn install_for_test(records: Vec<DeviceRecord>) {
    *TABLE.write() = records;
}

#[cfg(test)]
pub(crate) fn count() -> usize {
    TABLE.read().len()
}

// Suppress unused-import warnings when the legacy class ids constants
// are referenced indirectly. Kept here so adding a new class above does
// not need touching every caller.
const _: u32 = class_ids::OTHER;
