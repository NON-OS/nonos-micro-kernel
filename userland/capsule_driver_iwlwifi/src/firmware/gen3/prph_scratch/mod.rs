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

//! The gen3 peripheral-scratch block the boot ROM reads via context info: the
//! control flags, the platform-NVM and reduce-power addresses, the receive-
//! buffer ring, and the firmware DRAM image map. Layout from `iwl_prph_scratch`
//! (Linux pcie/iwl-context-info-v2.h): a packed, little-endian 1724-byte struct.

pub mod flags;
mod le;
pub mod layout;
mod scratch;
mod write;

pub use layout::{FSEQ_ENTRIES, MAX_DRAM_ENTRY, PRPH_SCRATCH_SIZE};
pub use scratch::{DramImage, PrphScratch};
