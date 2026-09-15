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

//! Section markers and placement granularity for the firmware image.

/// Section load offset marking the LMAC/UMAC boundary (`CPU1_CPU2_SEPARATOR`).
pub(super) const SEP_CPU1_CPU2: u32 = 0xFFFF_CCCC;
/// Section load offset marking the boundary before the paged image
/// (`PAGING_SEPARATOR_SECTION`).
pub(super) const SEP_PAGING: u32 = 0xAAAA_BBBB;
/// Each firmware chunk is placed on its own page, as the Linux driver allocates
/// one coherent block per section.
pub(super) const CHUNK_ALIGN: u64 = 4096;
