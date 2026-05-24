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

//! Wire-form ABI types for the driver broker syscall surface. The
//! kernel asserts these layouts at compile time on its side
//! (`src/hardware/broker/device.rs`, `src/syscall/microkernel/*`);
//! the asserts here keep the userland view in lockstep.

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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Bar {
    pub base: u64,
    pub size: u64,
    pub kind: u8,
    pub flags: u8,
    pub _pad: [u8; 6],
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
    // Valid BAR slot span. Entries are not compacted; sparse PCI BAR
    // layouts keep their architectural BAR index.
    pub bar_count: u8,
    pub irq_line: u8,
    pub irq_pin: u8,
    pub _pad1: [u8; 1],
    pub irq_source: u32,
    pub bars: [Bar; 6],
    // Modern virtio register locations from the device's PCI vendor caps
    // (virtio_present 0 = none). Kernel-parsed because capsules cannot read
    // PCI config space; used to map the correct BAR+offset.
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MmioMapOut {
    pub user_va: u64,
    pub length: u64,
    pub grant_id: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrqBindOut {
    pub grant_id: u64,
    pub vector: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrqPollOut {
    pub seq: u64,
    pub overflow: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PioGrantOut {
    pub port_base: u16,
    pub port_count: u16,
    pub _pad: u32,
    pub grant_id: u64,
}

const _: () = assert!(core::mem::size_of::<Bar>() == 24);
const _: () = assert!(core::mem::size_of::<DeviceRecord>() == 200);
const _: () = assert!(core::mem::size_of::<MmioMapOut>() == 24);
const _: () = assert!(core::mem::size_of::<IrqBindOut>() == 16);
const _: () = assert!(core::mem::size_of::<IrqPollOut>() == 16);
const _: () = assert!(core::mem::size_of::<DmaMapOut>() == 32);
const _: () = assert!(core::mem::size_of::<PioGrantOut>() == 16);
