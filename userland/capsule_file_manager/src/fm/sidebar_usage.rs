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

extern crate alloc;

use alloc::string::{String, ToString};

use super::human_size::human_size;
use super::state::State;

/// How full the store is, as a percentage of the only ceiling the vfs reports:
/// the file-slot count. There is no byte capacity in `OP_USAGE`, so the bar
/// cannot be a share of bytes and does not pretend to be one.
pub fn fill_pct(state: &State) -> u32 {
    match state.usage {
        Some((files, _, max)) if max > 0 => (files.min(max) as u64 * 100 / max as u64) as u32,
        _ => 0,
    }
}

/// The line under the bar: the slot count the bar actually measures, with the
/// bytes in use stated separately because they are scaled against nothing.
pub fn occupancy(state: &State) -> String {
    match state.usage {
        Some((files, bytes, max)) if max > 0 => {
            alloc::format!("{files} of {max} files · {} used", human_size(bytes))
        }
        Some((files, bytes, _)) => {
            alloc::format!("{files} files · {} used", human_size(bytes))
        }
        None => "usage unavailable".to_string(),
    }
}
