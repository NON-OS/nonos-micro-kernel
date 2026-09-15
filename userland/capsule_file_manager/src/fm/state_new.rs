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
use alloc::vec::Vec;

use super::state::{Mode, SortMode, State, ViewKind};

impl State {
    pub fn new() -> Self {
        State {
            owner_pid: 0,
            prefix: String::from("/"),
            all: Vec::new(),
            entries: Vec::new(),
            cursor: 0,
            scroll: 0,
            preview: None,
            status: b"loading...",
            mode: Mode::Browse,
            input: String::new(),
            filter: String::new(),
            sort_mode: SortMode::Name,
            selected: Vec::new(),
            clipboard: Vec::new(),
            view_rows: super::layout::LIST_VISIBLE,
            row_top: super::layout::FIRST_ROW_Y,
            row_h: super::layout::ROW_H,
            view: ViewKind::Grid,
            grid_cols: 5,
            win_w: super::manifest::WIDTH,
            win_h: super::manifest::HEIGHT,
            dir_info: None,
            usage: None,
            tags: super::tags::TagMap::default(),
            tag_filter: String::new(),
            screen: super::screen::Screen::Browse,
            favorites: super::favorites::Favorites::default(),
            prefs: super::prefs::Prefs::default(),
            undo: super::undo::UndoStack::default(),
            query: String::new(),
            hits: Vec::new(),
            hit_filter: None,
            recents_filter: None,
            recents: Vec::new(),
            place_stats: Vec::new(),
        }
    }
}
