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

use nonos_app_skeleton::app::EventOutcome;
use nonos_libc::mk_uptime_ms;

use super::action::Action;
use super::navigate::navigate;
use crate::app::VideoApp;
use crate::player::{frame_after_delta, frame_at_permille, Clock};

const MAX_VOLUME: i32 = 100;

pub fn apply(app: &mut VideoApp, action: Action) -> EventOutcome {
    if action == Action::Close {
        return EventOutcome::Close;
    }
    if let Some(outcome) = navigate(app, action) {
        return outcome;
    }
    let Some(file) = app.file.as_ref() else {
        return EventOutcome::Idle;
    };
    let total = file.index.len() as u32;
    let upf = file.header.micro_sec_per_frame;
    match action {
        Action::TogglePlay => {
            app.playing = !app.playing;
            if app.playing {
                rebase(app, app.next, upf);
            }
            EventOutcome::Repaint
        }
        Action::SeekBy(d) => {
            let target = frame_after_delta(app.next, total, upf, d);
            rebase(app, target, upf);
            EventOutcome::Repaint
        }
        Action::SeekToPermille(p) => {
            rebase(app, frame_at_permille(total, p), upf);
            EventOutcome::Repaint
        }
        Action::Restart => {
            rebase(app, 0, upf);
            EventOutcome::Repaint
        }
        Action::VolumeBy(d) => {
            app.volume = (app.volume as i32 + d).clamp(0, MAX_VOLUME) as u32;
            EventOutcome::Repaint
        }
        Action::ToggleMute => {
            app.muted = !app.muted;
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}

fn rebase(app: &mut VideoApp, target: u32, usec_per_frame: u32) {
    let base = Clock::new(0, 0, usec_per_frame).pts_ms(target);
    app.clock = Clock::new(mk_uptime_ms(), base, usec_per_frame);
    app.next = target;
    app.force_decode = true;
}
