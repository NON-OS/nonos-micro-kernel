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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::input::on_event;
use super::manifest::manifest;
use super::paint::paint;
use super::state::Game;
use super::step::step;

pub struct SnakeApp {
    game: Game,
}

impl SnakeApp {
    pub fn new() -> Self {
        SnakeApp { game: Game::new() }
    }
}

impl App for SnakeApp {
    fn manifest(&self) -> AppManifest {
        manifest()
    }
    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(&mut self.game, event)
    }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        paint(&self.game, fb);
    }
    fn on_tick(&mut self) -> bool {
        step(&mut self.game)
    }
    fn tick_interval_ms(&self) -> i64 {
        self.game.interval_ms
    }
}
