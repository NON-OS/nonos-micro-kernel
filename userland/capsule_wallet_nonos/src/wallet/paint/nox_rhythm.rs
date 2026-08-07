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

use super::nox_layout::NoxLayout;

/// Vertical rhythm of the staking screen. Kept beside the widths rather than
/// inside them because the only thing that moves vertically is what happens
/// when the two columns stack.
impl NoxLayout {
    /// Top of the staking card when there is room to breathe.
    pub const TOP_ROOMY: u32 = 418;
    /// The least it may be pulled up to on a short window, still clear of the
    /// summary cards above it.
    pub const TOP_TIGHT: u32 = 300;
    /// Height of the staking card.
    pub const CARD_H: u32 = 236;
    /// Height of the rewards card, and of the lock card under it.
    pub const REWARD_H: u32 = 116;
    pub const LOCK_H: u32 = 96;

    /// Top of the staking card for this viewport. A short window pulls the
    /// card up rather than letting the content below it fall off the bottom,
    /// which is what a fixed top did: the fee note was sliced in half by the
    /// window edge and there was no way to reach it.
    pub fn top(&self) -> u32 {
        if self.height == 0 {
            return Self::TOP_ROOMY;
        }
        let needed = Self::CARD_H + 16 + 92 + 24;
        if self.height > Self::TOP_ROOMY + needed {
            Self::TOP_ROOMY
        } else {
            self.height.saturating_sub(needed).max(Self::TOP_TIGHT)
        }
    }

    pub fn tabs_y(&self) -> u32 {
        self.top() + 20
    }
    pub fn track_y(&self) -> u32 {
        self.top() + 110
    }
    pub fn action_y(&self) -> u32 {
        self.top() + 178
    }
    /// Top of the right column, which drops below the card when stacked.
    pub fn right_y(&self) -> u32 {
        if self.stacked {
            self.top() + Self::CARD_H + 16
        } else {
            self.top()
        }
    }
    /// Top of the lock card, under the rewards card in both arrangements.
    pub fn lock_y(&self) -> u32 {
        self.right_y() + Self::REWARD_H + 12
    }
    /// Top of the full-width fee note, clear of whatever came before it.
    pub fn fees_y(&self) -> u32 {
        let after_right = self.lock_y() + Self::LOCK_H + 16;
        if self.stacked {
            after_right
        } else {
            (self.top() + Self::CARD_H + 14).max(after_right)
        }
    }
}
