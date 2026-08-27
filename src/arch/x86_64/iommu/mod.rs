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

//! VT-d DMA remapping.
//!
//! `is_present` means firmware described an IOMMU. `is_enforcing` means one is
//! translating with this kernel's tables, and it is what every call claiming
//! to confine a device checks. When bring-up fails, calls return
//! `NotEnforcing` rather than success.

pub mod device;
pub mod domain;
pub mod globals;
pub mod mapping;
pub mod regs;
pub mod tables;
pub mod types;
pub mod unit;
