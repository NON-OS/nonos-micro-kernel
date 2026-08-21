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

/// Second-level paging depths a unit supports, from SAGAW.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgawLevels {
    Three,
    Four,
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

/// Prefers four levels: that covers a 48-bit space. Falls back to three,
/// then five.
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
