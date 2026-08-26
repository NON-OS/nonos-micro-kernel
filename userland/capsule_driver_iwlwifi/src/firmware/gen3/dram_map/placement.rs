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

//! The device-physical address of every firmware chunk after staging.

use alloc::vec::Vec;

/// Each firmware chunk's device address, grouped by image. The addresses go
/// straight into the `iwl_prph_scratch` DRAM arrays.
pub struct DramPlacement {
    /// LMAC chunk device addresses, in load order.
    pub lmac: Vec<u64>,
    /// UMAC chunk device addresses, in load order.
    pub umac: Vec<u64>,
    /// Paged chunk device addresses, in load order.
    pub virt: Vec<u64>,
    /// Total bytes written into the staging region.
    pub staged_bytes: usize,
}
