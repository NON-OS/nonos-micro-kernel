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

//! Decoding the Capability register of a remapping unit. Everything the rest
//! of the driver needs to know about what a unit supports comes from here, so
//! it holds no state and is checked against the spec's encodings host side.

/// Second-level paging levels a unit supports, from the SAGAW field. A unit
/// must support at least one or it cannot translate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgawLevels {
    /// 39-bit address width, three levels.
    Three,
    /// 48-bit address width, four levels.
    Four,
    /// 57-bit address width, five levels.
    Five,
}

impl AgawLevels {
    pub const fn page_table_levels(self) -> u8 {
        match self {
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
        }
    }

    /// The AGAW value written into a context entry for this depth.
    pub const fn context_aw(self) -> u8 {
        match self {
            Self::Three => 1,
            Self::Four => 2,
            Self::Five => 3,
        }
    }
}

/// Number of domains a unit supports, from the ND field: 2^(4 + 2*ND).
/// ND of 7 is reserved and reports none.
pub const fn domain_count(cap: u64) -> u32 {
    let nd = (cap & 0x7) as u32;
    if nd >= 7 {
        0
    } else {
        1u32 << (4 + 2 * nd)
    }
}

/// Widest input address the unit accepts, from MGAW, which stores width - 1.
pub const fn max_address_width(cap: u64) -> u8 {
    (((cap >> 16) & 0x3F) as u8) + 1
}

/// Deepest page-table depth the unit supports. Prefers four levels because
/// that is what covers a 48-bit space; falls back to three, then five.
pub const fn preferred_levels(cap: u64) -> Option<AgawLevels> {
    let sagaw = (cap >> 8) & 0x1F;
    if sagaw & (1 << 2) != 0 {
        Some(AgawLevels::Four)
    } else if sagaw & (1 << 1) != 0 {
        Some(AgawLevels::Three)
    } else if sagaw & (1 << 3) != 0 {
        Some(AgawLevels::Five)
    } else {
        None
    }
}

/// Required Write-Buffer Flushing. Units that set it need an explicit flush
/// after a table write before the hardware is guaranteed to see it.
pub const fn requires_write_buffer_flush(cap: u64) -> bool {
    cap & (1 << 4) != 0
}

/// Caching Mode. When set, the unit caches not-present entries, so any change
/// that creates a mapping has to be followed by an invalidation.
pub const fn caching_mode(cap: u64) -> bool {
    cap & (1 << 7) != 0
}

/// Byte offset of the fault-recording registers, stored as a 16-byte count.
pub const fn fault_recording_offset(cap: u64) -> usize {
    (((cap >> 24) & 0x3FF) as usize) * 16
}

/// Number of fault-recording registers, stored as count - 1.
pub const fn fault_recording_count(cap: u64) -> u16 {
    (((cap >> 40) & 0xFF) as u16) + 1
}
