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

//! No I/O port space and no window to stand in for one.

macro_rules! absent {
    ($read:ident, $write:ident, $ty:ty) => {
        /// # Safety
        ///
        /// Caller owns the device answering at `port`.
        #[inline]
        pub unsafe fn $read(_port: u16) -> $ty {
            !0
        }

        /// # Safety
        ///
        /// Caller owns the device answering at `port`.
        #[inline]
        pub unsafe fn $write(_port: u16, _value: $ty) {}
    };
}

absent!(inb, outb, u8);
absent!(inw, outw, u16);
absent!(inl, outl, u32);
