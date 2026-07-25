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

use crate::model::PlayerView;
use crate::waveform::Waveform;

use super::draw::{paint_bar, paint_buttons, paint_waveform, ACCENT, BG, MUTED, TEXT};
use super::format::{state_label, time_readout};
use super::geometry::Layout;

pub fn paint_player(fb: &mut PaintBuffer, v: &PlayerView, wf: &Waveform, l: &Layout) {
    fb.clear(BG);
    fb.text_scaled(24, 20, v.title.as_bytes(), TEXT, 2);
    fb.text(24, 56, v.artist.as_bytes(), MUTED);
    fb.text(24, 72, v.format.as_bytes(), MUTED);
    let mut buf = [0u8; 16];
    let n = time_readout(&mut buf, v.pos_ms, v.dur_ms);
    fb.text(288, 24, &buf[..n], TEXT);
    fb.text(288, 44, state_label(v.state), ACCENT);
    paint_waveform(fb, v, wf, &l.waveform);
    paint_bar(fb, &l.progress, v.pos_ms, v.dur_ms);
    paint_bar(fb, &l.volume, v.volume_q15.max(0) as u32, 0x8000);
    paint_buttons(fb, v, l);
}
