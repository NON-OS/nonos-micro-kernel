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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainId(u16);

impl DomainId {
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    pub const fn as_u16(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(u16);

impl SourceId {
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    pub const fn as_u16(&self) -> u16 {
        self.0
    }

    pub const fn bus(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    pub const fn device(&self) -> u8 {
        ((self.0 >> 3) & 0x1F) as u8
    }

    pub const fn function(&self) -> u8 {
        (self.0 & 0x7) as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IoVirtAddr(u64);

impl IoVirtAddr {
    pub const fn new(addr: u64) -> Self {
        Self(addr)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    pub const fn is_page_aligned(&self) -> bool {
        self.0 & (PAGE_SIZE_4K as u64 - 1) == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IommuPageFlags {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
    pub snoop: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VtdError {
    NotPresent,
    /// Hardware was found but translation is never enabled, so nothing here
    /// confines a device. Returned instead of success.
    NotEnforcing,
    DomainTableFull,
    DomainAlreadyExists,
    DomainNotFound,
    DeviceAlreadyAttached,
    DeviceNotAttached,
    AddressMisaligned,
    SizeMisaligned,
    RangeOutOfBounds,
    PageTableExhausted,
    RangeAlreadyMapped,
    RangeNotMapped,
}

pub const PAGE_SHIFT_4K: u32 = 12;
pub const PAGE_SIZE_4K: usize = 1 << PAGE_SHIFT_4K;
pub const PAGE_MASK_4K: u64 = !((PAGE_SIZE_4K as u64) - 1);

pub const MAX_VTD_DOMAINS: usize = 256;
pub const MAX_VTD_DEVICES: usize = 256;
pub const MAX_VTD_MAPPINGS_PER_DOMAIN: usize = 4096;
