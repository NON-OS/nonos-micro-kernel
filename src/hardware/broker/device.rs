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

// Wire-form view of a device, returned by `MkDeviceList`. Layout is
// fixed; field order matches `abi/driver_broker_abi.md`. The struct
// is `repr(C)` and `Copy` so it can be written directly into a user
// buffer with usercopy.

pub const BUS_KIND_PCI: u8 = 1;
pub const BUS_KIND_ACPI: u8 = 2;
pub const BUS_KIND_VIRT: u8 = 3;

pub const BAR_KIND_NONE: u8 = 0;
pub const BAR_KIND_MMIO: u8 = 1;
pub const BAR_KIND_PIO: u8 = 2;

pub const BAR_FLAG_PREFETCH: u8 = 1 << 0;
pub const BAR_FLAG_MEM64: u8 = 1 << 1;

pub const DEVICE_FLAG_CLAIMED: u32 = 1 << 0;
pub const DEVICE_FLAG_DISABLED: u32 = 1 << 1;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusKind {
    Pci = BUS_KIND_PCI,
    Acpi = BUS_KIND_ACPI,
    Virt = BUS_KIND_VIRT,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarKind {
    None = BAR_KIND_NONE,
    Mmio = BAR_KIND_MMIO,
    Pio = BAR_KIND_PIO,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Bar {
    pub base: u64,
    pub size: u64,
    pub kind: u8,
    pub flags: u8,
    pub _pad: [u8; 6],
}

impl Bar {
    pub const fn empty() -> Self {
        Self { base: 0, size: 0, kind: BAR_KIND_NONE, flags: 0, _pad: [0; 6] }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceRecord {
    pub device_id: u64,
    pub bus_kind: u8,
    pub _pad0: [u8; 3],
    pub class: u32,
    pub vendor: u16,
    pub device: u16,
    pub flags: u32,
    // Number of BAR slots in the valid index span. PCI BARs are not
    // compressed: a device may expose BAR4 while BAR2/BAR3 are empty,
    // and userland must still pass BAR4 to MkMmioMap / MkPioGrant.
    pub bar_count: u8,
    pub irq_line: u8,
    pub irq_pin: u8,
    pub _pad1: [u8; 1],
    // Broker-published interrupt source. On x86 the broker derives
    // INTx routing from `irq_line` and ignores this; on aarch64 it
    // is the GIC SPI intid; on riscv64 it is the PLIC source id.
    // Zero means "no IRQ published for this device" — the IRQ broker
    // bind path refuses such devices.
    pub irq_source: u32,
    pub bars: [Bar; 6],
    // Modern virtio register locations parsed from the device's PCI vendor
    // caps (virtio_present 0 = none). A capsule cannot read PCI config space,
    // so it relies on these to map the correct BAR+offset for the common /
    // notify / device configuration structures.
    pub virtio_present: u8,
    pub virtio_common_bar: u8,
    pub virtio_notify_bar: u8,
    pub virtio_device_bar: u8,
    pub virtio_common_off: u32,
    pub virtio_notify_off: u32,
    pub virtio_device_off: u32,
    pub virtio_isr_off: u32,
    pub virtio_notify_mult: u32,
}

impl DeviceRecord {
    pub const fn empty() -> Self {
        Self {
            device_id: 0,
            bus_kind: 0,
            _pad0: [0; 3],
            class: 0,
            vendor: 0,
            device: 0,
            flags: 0,
            bar_count: 0,
            irq_line: 0xFF,
            irq_pin: 0,
            _pad1: [0; 1],
            irq_source: 0,
            bars: [Bar::empty(); 6],
            virtio_present: 0,
            virtio_common_bar: 0,
            virtio_notify_bar: 0,
            virtio_device_bar: 0,
            virtio_common_off: 0,
            virtio_notify_off: 0,
            virtio_device_off: 0,
            virtio_isr_off: 0,
            virtio_notify_mult: 0,
        }
    }
}

// Compile-time guarantee that the wire layout is what
// `driver_broker_abi.md` promises.
const _: () = {
    assert!(core::mem::size_of::<Bar>() == 24);
    assert!(core::mem::size_of::<DeviceRecord>() == 200);
};
