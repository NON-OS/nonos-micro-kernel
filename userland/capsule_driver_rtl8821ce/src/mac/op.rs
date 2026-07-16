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

//! One MAC-initialisation register operation. rtw88's `rtw8821c_mac_init` is a
//! linear register program that writes whole 8-, 16- and 32-bit registers and
//! sets or clears bits in them (`rtw_write8/16/32`, `rtw_write8/32_set`,
//! `rtw_write8/32_clr`). This encodes exactly that: an operation is a width, a
//! kind (write the value, OR the value in, or AND its complement in) and an
//! offset. A masked field update (clear a field then write it) is two ops. The
//! init table is an array of these the executor walks in order; the shape lets
//! the exact register program be checked against a modeled device off-silicon.

/// Register access width in bytes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Width {
    B8,
    B16,
    B32,
}

/// What an operation does to the register at its offset.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// Write `value` over the whole register.
    Write,
    /// Set the bits of `value`: `reg |= value`.
    Set,
    /// Clear the bits of `value`: `reg &= !value`.
    Clear,
}

/// One register operation in a MAC init table.
#[derive(Clone, Copy)]
pub struct MacOp {
    pub offset: u16,
    pub width: Width,
    pub kind: Kind,
    pub value: u32,
}

impl MacOp {
    pub const fn write8(offset: u16, value: u8) -> Self {
        Self { offset, width: Width::B8, kind: Kind::Write, value: value as u32 }
    }
    pub const fn write16(offset: u16, value: u16) -> Self {
        Self { offset, width: Width::B16, kind: Kind::Write, value: value as u32 }
    }
    pub const fn write32(offset: u16, value: u32) -> Self {
        Self { offset, width: Width::B32, kind: Kind::Write, value }
    }
    pub const fn set8(offset: u16, value: u8) -> Self {
        Self { offset, width: Width::B8, kind: Kind::Set, value: value as u32 }
    }
    pub const fn set32(offset: u16, value: u32) -> Self {
        Self { offset, width: Width::B32, kind: Kind::Set, value }
    }
    pub const fn clear8(offset: u16, value: u8) -> Self {
        Self { offset, width: Width::B8, kind: Kind::Clear, value: value as u32 }
    }
    pub const fn clear32(offset: u16, value: u32) -> Self {
        Self { offset, width: Width::B32, kind: Kind::Clear, value }
    }
}
