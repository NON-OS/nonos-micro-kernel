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

use super::recents_group::parent_of;
use super::refresh::refresh;
use super::screen::Screen;
use super::scroll::ensure_visible;
use super::state::State;

/// Show `path` in Browse. A directory becomes the listed prefix; a file lists
/// its parent and puts the cursor on it, which is what every screen outside
/// Browse means by "open this".
pub fn navigate(state: &mut State, path: &str) {
    state.screen = Screen::Browse;
    let dir = path.ends_with('/');
    let prefix = if dir { path } else { parent_of(path) };
    if state.prefix.as_str() != prefix {
        state.prefix = String::from(prefix);
    }
    state.cursor = 0;
    state.scroll = 0;
    refresh(state);
    if dir {
        return;
    }
    if let Some(index) = state.entries.iter().position(|e| e.full_path == path) {
        state.cursor = index;
        ensure_visible(state);
    }
}
