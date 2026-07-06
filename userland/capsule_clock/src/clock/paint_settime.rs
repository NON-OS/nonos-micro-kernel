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
    let buf = fmt::hm(state.edit_hour, state.edit_min);
    fb.text_scaled(96, 100, &buf, theme::FG, 6);
    adj(fb, 40, b"H-");
    adj(fb, 118, b"H+");
    adj(fb, 202, b"M-");
    adj(fb, 280, b"M+");
    fb.fill_rect(40, 300, 280, 48, theme::ACCENT);
    fb.text(150, 318, b"Apply", theme::BG);
}

fn adj(fb: &mut PaintBuffer, x: u32, label: &[u8]) {
    fb.fill_rect(x, 210, 68, 48, theme::DIM);
    fb.text(x + 22, 228, label, theme::BG);
}
