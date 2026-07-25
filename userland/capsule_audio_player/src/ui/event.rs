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

use super::control::{control_at, Control};
use super::Layout;
use crate::resample::OUT_RATE;
use crate::transport::{State, Transport};
use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind, KEY_ENTER, KEY_LEFT, KEY_RIGHT};

const KEY_SPACE: u32 = 0x20;

pub fn handle(tp: &mut Transport, l: &Layout, ev: InputEvent) -> EventOutcome {
    match ev.kind {
        InputKind::ButtonDown => match control_at(l, ev.x, ev.y) {
            Some(c) => {
                apply(tp, c);
                EventOutcome::Repaint
            }
            None => EventOutcome::Idle,
        },
        InputKind::KeyDown => match ev.code {
            KEY_SPACE | KEY_ENTER => {
                apply(tp, Control::PlayPause);
                EventOutcome::Repaint
            }
            KEY_LEFT => {
                apply(tp, Control::SeekBackSecs(10));
                EventOutcome::Repaint
            }
            KEY_RIGHT => {
                apply(tp, Control::SeekFwdSecs(10));
                EventOutcome::Repaint
            }
            _ => EventOutcome::Idle,
        },
        _ => EventOutcome::Idle,
    }
}

fn apply(tp: &mut Transport, c: Control) {
    match c {
        Control::PlayPause => {
            if tp.state() == State::Playing {
                tp.pause()
            } else {
                tp.play()
            }
        }
        Control::Stop => tp.stop(),
        Control::SeekBackSecs(s) => {
            tp.seek_frames(tp.pos_frames().saturating_sub(s as u64 * OUT_RATE as u64))
        }
        Control::SeekFwdSecs(s) => tp.seek_frames(tp.pos_frames() + s as u64 * OUT_RATE as u64),
        Control::SeekPermille(p) => tp.seek_frames(tp.dur_frames() * p as u64 / 1000),
        Control::VolumePermille(p) => tp.set_volume((0x8000i32 * p as i32) / 1000),
    }
}
