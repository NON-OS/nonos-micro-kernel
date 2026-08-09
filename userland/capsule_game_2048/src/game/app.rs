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
use nonos_libc::{mk_exit, mk_time_millis};
use tools_2048::Game;

pub struct Game2048 {
    game: Game<4>,
    over: bool,
}

pub fn new_board() -> Game<4> {
    match Game::<4>::new_seeded(mk_time_millis() as u64) {
        Ok(game) => game,
        Err(_) => mk_exit(1),
    }
}

impl Game2048 {
    pub fn new() -> Self {
        Game2048 { game: new_board(), over: false }
    }
}

impl App for Game2048 {
    fn manifest(&self) -> AppManifest {
        super::manifest::manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        super::event::on_event(event, &mut self.game, &mut self.over)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        super::paint::paint(fb, &self.game, self.over);
    }
}
