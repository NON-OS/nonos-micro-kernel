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

use nonos_app_skeleton::{
    EventOutcome, InputEvent, InputKind, KEY_DOWN, KEY_ESC, KEY_LEFT, KEY_RIGHT, KEY_UP,
};
use tools_2048::{Game, GameMove, GameState};

pub fn on_event(event: InputEvent, game: &mut Game<4>, over: &mut bool) -> EventOutcome {
    if event.kind != InputKind::KeyDown {
        return EventOutcome::Idle;
    }
    let mv = match event.code {
        KEY_LEFT => GameMove::Left,
        KEY_RIGHT => GameMove::Right,
        KEY_UP => GameMove::Up,
        KEY_DOWN => GameMove::Down,
        KEY_ESC => return EventOutcome::Close,
        0x52 | 0x72 => {
            *game = super::app::new_board();
            *over = false;
            return EventOutcome::Repaint;
        }
        _ => return EventOutcome::Idle,
    };
    if *over {
        return EventOutcome::Idle;
    }
    let changed = game.make_move(mv);
    *over = matches!(game.state(), GameState::GameOver);
    if changed {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}
