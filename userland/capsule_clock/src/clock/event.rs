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

use crate::clock::manifest::WIDTH;
use crate::clock::state::State;
use crate::clock::tabs::{self, Tab};

pub fn on_click(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if let Some(t) = tabs::hit(WIDTH as i32, x, y) {
        state.tab = t;
        return EventOutcome::Repaint;
    }
    if state.tab == Tab::Stopwatch {
        return stopwatch_click(state, x, y);
    }
    if state.tab == Tab::Timer {
        return timer_click(state, x, y);
    }
    EventOutcome::Idle
}

fn timer_click(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if in_rect(x, y, 40, 250, 80, 40) {
        state.timer.set(60_000);
    } else if in_rect(x, y, 140, 250, 80, 40) {
        state.timer.set(300_000);
    } else if in_rect(x, y, 240, 250, 80, 40) {
        state.timer.set(600_000);
    } else if in_rect(x, y, 40, 320, 130, 46) {
        state.timer.toggle(state.now_ms);
    } else if in_rect(x, y, 190, 320, 130, 46) {
        state.timer.reset();
    } else {
        return EventOutcome::Idle;
    }
    EventOutcome::Repaint
}

fn stopwatch_click(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if in_rect(x, y, 40, 300, 130, 46) {
        state.sw.toggle(state.now_ms);
        return EventOutcome::Repaint;
    }
    if in_rect(x, y, 190, 300, 130, 46) {
        state.sw.reset();
        return EventOutcome::Repaint;
    }
    EventOutcome::Idle
}

fn in_rect(x: i32, y: i32, rx: i32, ry: i32, w: i32, h: i32) -> bool {
    x >= rx && x < rx + w && y >= ry && y < ry + h
}
