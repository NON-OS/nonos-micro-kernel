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

use crate::pm::state::{Screen, State};

use super::table_geom::{Col, COLS_FULL, COLS_OVERVIEW};
use super::{chrome, insp_geom, nav_geom, search};

#[path = "hit_pane.rs"]
mod hit_pane;

pub use hit_pane::rows_visible;

// What the pointer landed on, named in the vocabulary of the surface that drew
// it. Every variant is produced by the same pure geometry function its painter
// read, so a target can never name a rect the frame did not draw.
pub enum Target {
    Nav(Screen),
    Sort(Col),
    Row(usize),
    Matrix(usize),
    Finding(usize),
    EndProcess,
    ForceQuit,
    Search,
}

// The one routing surface. The sidebar and the inspector are window-global and
// are tested first; everything else is pane-local, so the pane origin comes off
// the coordinate exactly once before any screen geometry sees it. The search
// field sits in the head band inside the inspector's column, so it has to be
// asked before the inspector claims that whole x-range.
pub fn at(state: &State, w: u32, h: u32, x: i32, y: i32) -> Option<Target> {
    if let Some(screen) = nav_geom::at(x, y) {
        return Some(Target::Nav(screen));
    }
    let (sx, sy, sw, sh) = search::rect(w);
    if x >= sx as i32 && x < (sx + sw) as i32 && y >= sy as i32 && y < (sy + sh) as i32 {
        return Some(Target::Search);
    }
    let inspector = state.screen.has_inspector();
    if inspector && x >= insp_geom::pane_x(w) as i32 {
        let index = insp_geom::btn_at(w, h, x, y)?;
        return Some(if index == 0 { Target::EndProcess } else { Target::ForceQuit });
    }
    let r = chrome::pane_rect(w, h, inspector);
    let x = x - r.x as i32;
    let y = y - r.y as i32;
    if x < 0 || y < 0 || x >= r.w as i32 || y >= r.h as i32 {
        return None;
    }
    match state.screen {
        Screen::Overview => hit_pane::table(state, &r, &COLS_OVERVIEW, x, y),
        Screen::Processes => hit_pane::table(state, &r, &COLS_FULL, x, y),
        Screen::Authority => hit_pane::matrix(state, &r, x, y),
        Screen::Security => hit_pane::finding(state, &r, y),
        _ => None,
    }
}
