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

use core::cmp::Ordering;

use alloc::vec::Vec;

use super::entries::Entry;
use super::filetype::{ext, kind_of};
use super::scroll::ensure_visible;
use super::state::{SortMode, State};

// Rebuild the visible list from the full listing: keep entries whose name
// contains the active filter, order them by the current sort mode, then keep
// the cursor and scroll window in range. Called whenever the listing, filter,
// or sort mode changes.
pub fn rebuild_view(state: &mut State) {
    let needle = state.filter.to_ascii_lowercase();
    let mut view: Vec<Entry> = state
        .all
        .iter()
        .filter(|e| needle.is_empty() || e.label.to_ascii_lowercase().contains(needle.as_str()))
        .cloned()
        .collect();
    sort_view(&mut view, state.sort_mode, state.sort_rev);
    state.entries = view;
    if state.cursor >= state.entries.len() {
        state.cursor = state.entries.len().saturating_sub(1);
    }
    ensure_visible(state);
}

// Directories always lead; within each group the chosen key decides order,
// falling back to name so the result is stable. `rev` flips the within-group
// order while keeping directories first.
fn sort_view(entries: &mut [Entry], mode: SortMode, rev: bool) {
    entries.sort_by(|a, b| match b.is_dir.cmp(&a.is_dir) {
        Ordering::Equal => {
            let ord = by_mode(a, b, mode);
            if rev {
                ord.reverse()
            } else {
                ord
            }
        }
        other => other,
    });
}

fn by_mode(a: &Entry, b: &Entry, mode: SortMode) -> Ordering {
    match mode {
        SortMode::Name => by_name(a, b),
        SortMode::Size => b.size.unwrap_or(0).cmp(&a.size.unwrap_or(0)).then_with(|| by_name(a, b)),
        SortMode::Date => b.mtime.cmp(&a.mtime).then_with(|| by_name(a, b)),
        SortMode::Type => ext(&a.label).cmp(ext(&b.label)).then_with(|| by_name(a, b)),
    }
    // A directory-vs-directory type sort keeps them together already; kind is
    // only meaningful for painting, so it stays out of the ordering.
    .then_with(|| kind_of(a).cmp(&kind_of(b)))
}

fn by_name(a: &Entry, b: &Entry) -> Ordering {
    a.label.to_ascii_lowercase().cmp(&b.label.to_ascii_lowercase())
}
