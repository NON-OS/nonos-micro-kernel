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

use super::control::Control;
use crate::resample::OUT_RATE;
use crate::transport::{State, Transport};
use nonos_app_skeleton::{EventOutcome, KEY_ENTER, KEY_LEFT, KEY_RIGHT};

const KEY_SPACE: u32 = 0x20;

pub fn apply(tp: &mut Transport, c: Control) {
    match c {
        Control::PlayPause => {
            if tp.state() == State::Playing {
                tp.pause()
            } else {
                tp.play()
            }
        }
        Control::Seek(p) => {
            tp.seek_frames(tp.dur_frames() * p as u64 / 1000);
        }
        Control::Volume(p) => tp.set_volume((0x8000i32 * p as i32) / 1000),
        Control::Mute => tp.toggle_mute(),
        Control::SeekBackSecs(s) => {
            tp.seek_frames(tp.pos_frames().saturating_sub(s as u64 * OUT_RATE as u64));
        }
        Control::SeekFwdSecs(s) => {
            tp.seek_frames(tp.pos_frames() + s as u64 * OUT_RATE as u64);
        }
        Control::Prev | Control::Next | Control::Shuffle | Control::Repeat => {}
    }
}

pub fn key(tp: &mut Transport, code: u32) -> EventOutcome {
    match code {
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
    }
}
