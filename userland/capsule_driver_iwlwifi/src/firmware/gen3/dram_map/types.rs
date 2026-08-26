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

//! The runtime image partitioned into its three sub-images, in load order.

use alloc::vec::Vec;

use super::super::image::Section;

/// The runtime sections grouped by which image they belong to.
pub struct FwLayout<'a> {
    /// LMAC sections: the run before the CPU1/CPU2 separator.
    pub lmac: Vec<Section<'a>>,
    /// UMAC sections: the run between the two separators.
    pub umac: Vec<Section<'a>>,
    /// Paged ("virtual") sections: the run after the paging separator.
    pub virt: Vec<Section<'a>>,
}
