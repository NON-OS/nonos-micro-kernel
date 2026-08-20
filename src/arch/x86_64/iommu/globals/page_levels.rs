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

use core::sync::atomic::Ordering;

use super::state::PAGE_LEVELS;

/// Depth the domains' tables are built to, or `None` before a unit was probed.
pub fn page_levels() -> Option<u8> {
    match PAGE_LEVELS.load(Ordering::Acquire) {
        0 => None,
        levels => Some(levels),
    }
}

/// Record the depth a probed unit reported. Set once, from the unit bring-up.
pub fn set_page_levels(levels: u8) {
    PAGE_LEVELS.store(levels, Ordering::Release);
}
