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

use nonos_app_skeleton::clients::vfs::dirstat;

use super::screen::Screen;
use super::state::State;

/// Every store prefix a Home card states a figure for, walked once and read
/// from cache thereafter.
pub const WALKED: [&str; 4] = ["/", "/capsules/", "/projects/", "/downloads/"];

/// Fill the walk cache when Home is the drawn surface and nothing is cached.
/// Paint must never block on IPC, so every count on the surface is fetched here
/// and a refresh drops the cache to force the next visit to re-walk.
pub fn sync_places(state: &mut State) {
    if state.screen != Screen::Home || !state.place_stats.is_empty() {
        return;
    }
    state.place_stats = WALKED
        .iter()
        .map(|p| (p.to_string(), dirstat(state.owner_pid, p.as_bytes()).ok()))
        .collect();
}

/// The cached walk for `path`: `None` when it is not cached yet or the walk
/// failed, which the callers state rather than substituting a zero for.
pub fn stat_of(state: &State, path: &str) -> Option<(u32, u32, u64, bool)> {
    state.place_stats.iter().find(|(key, _)| key == path).and_then(|(_, stat)| *stat)
}

/// A category card's one-line figure. A truncated walk counted a floor rather
/// than a total and is marked as one.
pub fn count_line(state: &State, path: &str) -> String {
    match stat_of(state, path) {
        Some((files, dirs, _, truncated)) if files + dirs > 0 => {
            let mark = if truncated { "over " } else { "" };
            alloc::format!("{mark}{} items", files + dirs)
        }
        Some(_) => "Empty".to_string(),
        None => "No count".to_string(),
    }
}

/// How much of the store's file count sits under `path`, which is the only
/// denominator the vfs actually reports; a missing total leaves the bar empty
/// rather than scaled against a number nobody has.
pub fn share_pct(state: &State, path: &str) -> u32 {
    match (stat_of(state, path), state.usage) {
        (Some((files, _, _, _)), Some((total, _, _))) if total > 0 => {
            (files.min(total) as u64 * 100 / total as u64) as u32
        }
        _ => 0,
    }
}
