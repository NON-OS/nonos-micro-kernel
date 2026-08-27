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

use nonos_app_skeleton::EventOutcome;

use crate::snake::state::{Game, Screen};

use super::target::{self, Target};
use super::{click_home, click_run, click_setup, nav};

pub fn on_click(game: &mut Game, x: i32, y: i32) -> EventOutcome {
    match target::at(game.screen, x, y) {
        Some(target) => apply(game, target),
        None => EventOutcome::Idle,
    }
}

// The index a geom module answered with is the action selector, so the tables
// in `click_*` line up one-for-one with the label tables in `ui/*_geom.rs`.
fn apply(game: &mut Game, target: Target) -> EventOutcome {
    match target {
        Target::HomeAction(index) => click_home::action(game, index),
        Target::HomeCard(index) => click_home::card(game, index),
        Target::Chip(row, index) => click_setup::chip(game, row, index),
        Target::Toggle(index) => click_setup::toggle(game, index),
        Target::Start => nav::start_run(game),
        Target::Foot(index) => click_run::foot(game, index),
        Target::PauseAction(index) => click_run::pause_action(game, index),
        Target::OverAction(index) => click_run::over_action(game, index),
        Target::RankBack => nav::go(game, Screen::Home),
    }
}
