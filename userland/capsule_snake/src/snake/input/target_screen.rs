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

use crate::snake::ui::{home_geom, rank_geom, setup_geom, setup_geom_rows};

use super::target::Target;

pub fn home(w: u32, h: u32, x: i32, y: i32) -> Option<Target> {
    if let Some(index) = home_geom::action_at(w, h, x, y) {
        return Some(Target::HomeAction(index));
    }
    home_geom::card_at(w, h, x, y).map(Target::HomeCard)
}

pub fn setup(w: u32, h: u32, x: i32, y: i32) -> Option<Target> {
    if let Some((row, index)) = setup_geom_rows::chip_at(w, h, x, y) {
        return Some(Target::Chip(row, index));
    }
    if let Some(index) = setup_geom_rows::toggle_at(w, h, x, y) {
        return Some(Target::Toggle(index));
    }
    if setup_geom::start_at(w, h, x, y) {
        return Some(Target::Start);
    }
    None
}

pub fn rank(w: u32, h: u32, x: i32, y: i32) -> Option<Target> {
    if rank_geom::back_at(w, h, x, y) {
        return Some(Target::RankBack);
    }
    None
}
