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

use alloc::string::String;

use nonos_app_skeleton::clients::vfs::copy;

use super::refresh::refresh;
use super::selection;
use super::state::State;

// Copy the acting set (selection or cursor) into the current directory under a
// non-colliding "(copy)" name, so an entry can be duplicated in place.
pub fn duplicate(state: &mut State) {
    let act = selection::acting(state);
    if act.is_empty() {
        state.status = b"nothing to duplicate";
        return;
    }
    let pid = state.owner_pid;
    let mut failed = false;
    for (full, is_dir) in &act {
        let src = full.trim_end_matches('/');
        let base = src.rsplit('/').next().unwrap_or("");
        let name = unique_name(state, base);
        let dest = alloc::format!("{}{}", state.prefix, name);
        if copy(pid, src.as_bytes(), dest.as_bytes(), *is_dir).is_err() {
            failed = true;
        }
    }
    selection::clear(state);
    refresh(state);
    state.status = if failed { b"duplicate: some failed" } else { b"duplicated" };
}

// Build "name (copy).ext", then "(copy 2)", "(copy 3)"... until it does not
// collide with an existing entry.
fn unique_name(state: &State, base: &str) -> String {
    let (stem, ext) = match base.rfind('.') {
        Some(i) if i > 0 => (&base[..i], &base[i..]),
        _ => (base, ""),
    };
    for n in 1..1000 {
        let tag = if n == 1 { String::from(" (copy)") } else { alloc::format!(" (copy {n})") };
        let candidate = alloc::format!("{stem}{tag}{ext}");
        if !state.all.iter().any(|e| e.label.trim_end_matches('/') == candidate) {
            return candidate;
        }
    }
    alloc::format!("{stem} (copy){ext}")
}
