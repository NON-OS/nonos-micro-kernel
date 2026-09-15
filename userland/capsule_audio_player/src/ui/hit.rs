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

//! One hit-test for the whole surface. Every branch calls the same geometry
//! function the painter used, so a click can only resolve to a control that
//! was actually drawn under the pointer.

use super::control::Control;
use super::geometry::{page, shell};
use super::hit_screen::{bar_hit, content_hit};
use super::shell as chrome;
use super::state::{UiState, View};

pub enum Action {
    Go(View),
    Ctl(Control),
    Select(usize),
    LibTab(usize),
    RailTab(usize),
    Playlist(usize),
    Section(usize),
    ClearQuery,
}

pub fn hit(
    ui: &UiState,
    dims: (u32, u32),
    rows: &[usize],
    n: usize,
    x: i32,
    y: i32,
) -> Option<Action> {
    let sh = shell(dims.0, dims.1);
    if sh.transport.contains(x, y) {
        return bar_hit(sh.transport, x, y);
    }
    if sh.sidebar.contains(x, y) {
        if let Some(v) = chrome::nav_at(sh.sidebar, x, y) {
            return Some(Action::Go(v));
        }
        return chrome::plist_at(sh.sidebar, x, y).map(Action::Playlist);
    }
    if sh.rail.contains(x, y) {
        if let Some(t) = chrome::rail_tab_at(sh.rail, x, y) {
            return Some(Action::RailTab(t));
        }
        return chrome::rail_queue_at(sh.rail, x, y).map(Action::Select);
    }
    if chrome::search_clear(sh.topbar).contains(x, y) && !ui.query.is_empty() {
        return Some(Action::ClearQuery);
    }
    if chrome::search_field(sh.topbar).contains(x, y) {
        return Some(Action::Go(View::Search));
    }
    content_hit(ui, page(&sh), rows, n, x, y)
}
