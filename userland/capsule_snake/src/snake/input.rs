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
    EventOutcome, InputEvent, KEY_DOWN, KEY_ENTER, KEY_LEFT, KEY_RIGHT, KEY_UP,
};

use super::state::{Dir, Game, Phase};

pub fn on_event(game: &mut Game, event: InputEvent) -> EventOutcome {
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    if let Some(dir) = direction(event.code) {
        return steer(game, dir);
    }
    match event.code {
        KEY_ENTER => restart(game),
        0x20 | 0x70 | 0x50 => toggle_pause(game),
        _ => EventOutcome::Idle,
    }
}

fn direction(code: u32) -> Option<Dir> {
    match code {
        KEY_UP | 0x77 | 0x57 => Some(Dir::Up),
        KEY_DOWN | 0x73 | 0x53 => Some(Dir::Down),
        KEY_LEFT | 0x61 | 0x41 => Some(Dir::Left),
        KEY_RIGHT | 0x64 | 0x44 => Some(Dir::Right),
        _ => None,
    }
}

fn steer(game: &mut Game, dir: Dir) -> EventOutcome {
    if dir == game.dir.opposite() {
        return EventOutcome::Idle;
    }
    match game.phase {
        Phase::Ready => {
            game.dir = dir;
            game.pending = dir;
            game.phase = Phase::Running;
            EventOutcome::Repaint
        }
        Phase::Running => {
            game.pending = dir;
            EventOutcome::Idle
        }
        Phase::Paused | Phase::GameOver => EventOutcome::Idle,
    }
}

fn restart(game: &mut Game) -> EventOutcome {
    if game.phase != Phase::GameOver {
        return EventOutcome::Idle;
    }
    game.reset();
    EventOutcome::Repaint
}

fn toggle_pause(game: &mut Game) -> EventOutcome {
    game.phase = match game.phase {
        Phase::Running => Phase::Paused,
        Phase::Paused => Phase::Running,
        Phase::Ready => Phase::Ready,
        Phase::GameOver => Phase::GameOver,
    };
    EventOutcome::Repaint
}
