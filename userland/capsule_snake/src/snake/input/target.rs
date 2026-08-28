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

use crate::snake::state::Screen;
use crate::snake::ui::{over_geom, pause_geom, play_geom_rows};

use super::surface;
use super::target_screen::{home, rank, setup};

// What the pointer landed on, named for the surface that drew it. Every arm
// below reads the same pure geom function its painter reads, at the surface
// size the painter published, so a hit box cannot drift from a shape.
#[derive(Clone, Copy)]
pub enum Target {
    HomeAction(usize),
    HomeCard(usize),
    Chip(usize, usize),
    Toggle(usize),
    Start,
    Foot(usize),
    PauseAction(usize),
    OverAction(usize),
    RankBack,
}

pub fn at(screen: Screen, x: i32, y: i32) -> Option<Target> {
    let (w, h) = surface::size();
    match screen {
        Screen::Home => home(w, h, x, y),
        Screen::Setup => setup(w, h, x, y),
        Screen::Play => play_geom_rows::foot_at(w, h, x, y).map(Target::Foot),
        Screen::Pause => pause_geom::action_at(w, h, x, y).map(Target::PauseAction),
        Screen::Over => over_geom::action_at(w, h, x, y).map(Target::OverAction),
        Screen::Rank => rank(w, h, x, y),
    }
}
