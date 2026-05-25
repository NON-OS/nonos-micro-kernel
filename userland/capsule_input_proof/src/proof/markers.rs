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

use nonos_app_skeleton::{InputEvent, InputKind};

use super::emit::Line;
use super::state::Latches;

pub fn on_input(latches: &mut Latches, event: &InputEvent) {
    announce_ready(latches);
    match event.kind {
        InputKind::KeyDown => on_key(latches, event),
        InputKind::PointerRel | InputKind::PointerAbs => on_motion(latches, event),
        InputKind::ButtonDown => on_click(latches, event),
        _ => {}
    }
    finish(latches);
}

fn announce_ready(latches: &mut Latches) {
    if !latches.ready {
        latches.ready = true;
        Line::new().push(b"surface ready").emit();
    }
}

fn on_key(latches: &mut Latches, event: &InputEvent) {
    if !latches.key {
        latches.key = true;
        Line::new().push(b"key down code=").num(event.code as i32).emit();
    }
    if latches.click && !latches.focus_routed {
        latches.focus_routed = true;
        Line::new().push(b"focus routed").emit();
    }
}

fn on_motion(latches: &mut Latches, event: &InputEvent) {
    if !latches.motion {
        latches.motion = true;
        Line::new().push(b"pointer motion x=").num(event.x).push(b" y=").num(event.y).emit();
    }
}

fn on_click(latches: &mut Latches, event: &InputEvent) {
    if !latches.click {
        latches.click = true;
        Line::new().push(b"click dispatch local=").num(event.x).push(b",").num(event.y).emit();
    }
}

fn finish(latches: &mut Latches) {
    if !latches.passed && latches.complete() {
        latches.passed = true;
        Line::new().push(b"PASS").emit();
    }
}
