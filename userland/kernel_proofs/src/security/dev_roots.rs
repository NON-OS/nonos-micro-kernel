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

//! `Authority`, restated with the kernel's variants and nothing else.
//!
//! The registry only ever matches on it to write one byte, so the shim needs
//! the variants and no behaviour. `registry_tests` pins that byte against the
//! reader's decoding.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authority {
    Vendor,
    Developer(u8),
    Publisher,
}
