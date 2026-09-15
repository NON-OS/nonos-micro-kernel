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

use alloc::string::String;

use super::human_size::human_size;
use super::selection_is_selected::is_selected;
use super::state::State;

/// How many entries are checked, or `None` when nothing is.
pub fn count_label(state: &State) -> Option<String> {
    match state.selected.len() {
        0 => None,
        1 => Some(String::from("1 selected")),
        n => Some(alloc::format!("{n} selected")),
    }
}

/// The band's headline: the count and the bytes behind it. Only files carry a
/// stat'd size -- a directory's is `None` and no walk is taken inside paint --
/// so a selection holding one is reported as a lower bound rather than as a
/// total the manager cannot actually stand behind.
pub fn summary(state: &State) -> String {
    let count = count_label(state).unwrap_or_default();
    let mut bytes = 0u64;
    let mut partial = false;
    for entry in state.entries.iter().filter(|e| is_selected(state, &e.full_path)) {
        match entry.size {
            Some(size) => bytes = bytes.saturating_add(size),
            None => partial = true,
        }
    }
    let size = human_size(bytes);
    if partial {
        alloc::format!("{count}  \u{b7}  at least {size}")
    } else {
        alloc::format!("{count}  \u{b7}  {size}")
    }
}
