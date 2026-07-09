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
    let e = state.sw.elapsed(state.now_ms);
    let buf = fmt::ms_clock(e);
    fb.text_scaled(40, 150, &buf, theme::FG, 5);
    let label: &[u8] = if state.sw.running { b"Stop" } else { b"Start" };
    fb.fill_rect(40, 300, 130, 46, theme::ACCENT);
    fb.text(80, 318, label, theme::BG);
    fb.fill_rect(190, 300, 130, 46, theme::DIM);
    fb.text(228, 318, b"Reset", theme::BG);
}
