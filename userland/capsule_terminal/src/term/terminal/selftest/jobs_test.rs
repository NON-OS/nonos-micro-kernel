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

use nonos_app_skeleton::{InputEvent, InputKind, MOD_CTRL};

use crate::event::on_event;
use crate::jobs::{self, JobWork};
use crate::term::state::State;

use super::{mark, visible_has};

const TICK_BUDGET: u32 = 8;

pub fn run(state: &mut State) {
    t_bg(state);
    t_cancel(state);
}

fn t_bg(state: &mut State) {
    state.scrollback.clear();
    let id = jobs::submit(state, b"echo hi", true, JobWork::Noop);
    let reaped = pump_until_gone(state, id);
    mark(b"bg", reaped && visible_has(state, b"[1] Done echo hi"));
}

fn t_cancel(state: &mut State) {
    state.scrollback.clear();
    let id = jobs::submit(state, b"ping 10.255.255.1", false, JobWork::Noop);
    state.fg_running = true;
    on_event(state, ctrl_c());
    let reaped = pump_until_gone(state, id);
    let cancelled = reaped && state.last_status == 130;
    mark(b"cancel", cancelled && visible_has(state, b"interrupted"));
}

fn pump_until_gone(state: &mut State, id: u32) -> bool {
    for _ in 0..TICK_BUDGET {
        jobs::pump(state);
        if state.jobs.get(id).is_none() {
            return true;
        }
    }
    false
}

fn ctrl_c() -> InputEvent {
    InputEvent {
        kind: InputKind::KeyDown,
        flags: MOD_CTRL,
        code: b'C' as u32,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    }
}
