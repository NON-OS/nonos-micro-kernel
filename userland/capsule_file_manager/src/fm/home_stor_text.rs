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

use super::home_count::{share_pct, stat_of};
use super::human_size::human_size;
use super::sidebar_usage::{fill_pct, occupancy};
use super::state::State;

/// How full the bar on card `idx` is drawn. The drive measures the file slots
/// the vfs reports a ceiling for; a place measures its share of those same
/// files. Neither is a share of bytes, because the store states no byte ceiling
/// to divide by.
pub fn bar_pct(state: &State, idx: usize, path: &str) -> u32 {
    if idx == 0 {
        fill_pct(state)
    } else {
        share_pct(state, path)
    }
}

/// The line under the bar, naming the count the bar actually measures.
pub fn detail(state: &State, idx: usize, path: &str) -> String {
    if idx == 0 {
        return occupancy(state);
    }
    match (stat_of(state, path), state.usage) {
        (Some((files, _, _, _)), Some((total, _, _))) if total > 0 => {
            alloc::format!("{files} of {total} stored files")
        }
        (Some((files, _, _, _)), _) => alloc::format!("{files} files · share unknown"),
        _ => "not counted".to_string(),
    }
}

/// The size stated on the title line: the store's own bytes-in-use for the
/// drive, the walked subtree's bytes for a place, and a dash when neither read
/// has landed rather than a zero that would read as empty.
pub fn size_text(state: &State, idx: usize, path: &str) -> String {
    let bytes = if idx == 0 {
        state.usage.map(|(_, used, _)| used)
    } else {
        stat_of(state, path).map(|(_, _, bytes, _)| bytes)
    };
    match bytes {
        Some(b) => human_size(b),
        None => "--".to_string(),
    }
}
