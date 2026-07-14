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

const BAR_KIND_NONE: u8 = 0;
pub(in crate::hardware::broker) const BAR_KIND_MMIO: u8 = 1;
pub(in crate::hardware::broker) const BAR_KIND_PIO: u8 = 2;

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
    /// Bus-specific auxiliary value. For an ACPI LPSS I2C controller it carries
    /// the DesignWare source clock in Hz; zero for every other bar. Placed here
    /// so its four-byte alignment keeps the record at 24 bytes.
    pub aux: u32,
    pub kind: u8,
    pub flags: u8,
    pub _pad: [u8; 2],
}

impl Bar {
    pub const fn empty() -> Self {
        Self { base: 0, size: 0, kind: BAR_KIND_NONE, flags: 0, aux: 0, _pad: [0; 2] }
    }
}

const _: () = assert!(core::mem::size_of::<Bar>() == 24);
