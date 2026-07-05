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

use nonos_app_skeleton::clients::vfs::{copy, rename};

use super::refresh::refresh;
use super::selection;
use super::state::State;

// A remembered entry awaiting a paste. `path` is keyed the way the store holds
// it (directories without a trailing slash); `cut` marks a move.
#[derive(Clone)]
pub struct Clip {
    pub path: String,
    pub is_dir: bool,
    pub cut: bool,
}

// Remember the acting set (selection, or the cursor) for a later paste, as a
// copy or a cut. Clears the selection so the next action starts fresh.
pub fn yank(state: &mut State, cut: bool) {
    let act = selection::acting(state);
    if act.is_empty() {
        state.status = b"nothing to yank";
        return;
    }
    state.clipboard = act
        .into_iter()
        .map(|(full, is_dir)| Clip { path: String::from(full.trim_end_matches('/')), is_dir, cut })
        .collect();
    selection::clear(state);
    state.status = if cut { b"cut (p to paste)" } else { b"copied (p to paste)" };
}

// Paste every clipboard entry into the current directory under its own name: a
// cut moves, a copy duplicates, a directory carries its whole subtree.
pub fn paste(state: &mut State) {
    if state.clipboard.is_empty() {
        state.status = b"clipboard empty";
        return;
    }
    let clips = state.clipboard.clone();
    let cut = clips.first().map(|c| c.cut).unwrap_or(false);
    let pid = state.owner_pid;
    let mut failed = false;
    for clip in &clips {
        let Some(base) = clip.path.rsplit('/').next().filter(|b| !b.is_empty()) else { continue };
        let dest = alloc::format!("{}{}", state.prefix, base);
        let result = if clip.cut {
            rename(pid, clip.path.as_bytes(), dest.as_bytes())
        } else {
            copy(pid, clip.path.as_bytes(), dest.as_bytes(), clip.is_dir)
        };
        if result.is_err() {
            failed = true;
        }
    }
    if cut {
        state.clipboard.clear();
    }
    refresh(state);
    state.status = if failed {
        b"paste: some failed"
    } else if cut {
        b"moved"
    } else {
        b"pasted"
    };
}
