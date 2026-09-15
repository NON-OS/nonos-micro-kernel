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

use alloc::string::ToString;

use super::home_cat_geom::cat_at;
use super::home_cats::CATS;
use super::home_stor_geom::{stor_at, STOR_PLACES};
use super::screen_hit::ScreenHit;
use super::state::State;

/// Home's two plate rows are laid out side by side, so unlike the stacked lists
/// they have to be tested on both axes. Both rows are searched from the very
/// cell lists their painters drew, which is what keeps a click on the card it
/// looks like it landed on.
pub fn home_hit(state: &State, x: u32, y: u32) -> Option<ScreenHit> {
    if let Some(idx) = cat_at(state.win_w, x, y) {
        let cat = &CATS[idx];
        return match (cat.path, cat.screen) {
            (Some(path), _) => Some(ScreenHit::Open(path.to_string())),
            (None, Some(screen)) => Some(ScreenHit::Go(screen)),
            (None, None) => None,
        };
    }
    stor_at(state.win_w, state.win_h, x, y)
        .map(|idx| ScreenHit::Open(STOR_PLACES[idx].1.to_string()))
}
