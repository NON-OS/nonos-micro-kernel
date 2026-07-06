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

use nonos_app_skeleton::PaintBuffer;

use crate::clock::state::State;
use crate::clock::{fmt, theme};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    if state.timer.fired {
        fb.fill_rect(0, 32, 360, 408, theme::ALERT);
    }
    let rem = state.timer.remaining(state.now_ms);
    let buf = fmt::ms_ms(rem);
    fb.text_scaled(84, 150, &buf, theme::FG, 6);
    preset(fb, 40, b"1m");
    preset(fb, 140, b"5m");
    preset(fb, 240, b"10m");
    let label: &[u8] = if state.timer.running { b"Pause" } else { b"Start" };
    fb.fill_rect(40, 320, 130, 46, theme::ACCENT);
    fb.text(80, 338, label, theme::BG);
    fb.fill_rect(190, 320, 130, 46, theme::DIM);
    fb.text(228, 338, b"Reset", theme::BG);
}

fn preset(fb: &mut PaintBuffer, x: u32, label: &[u8]) {
    fb.fill_rect(x, 250, 80, 40, theme::DIM);
    fb.text(x + 26, 264, label, theme::BG);
}
