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
use crate::transport::State;
use crate::ui::geometry::{Layout, Rect};
use crate::ui::sprite::{glow_disc, next, pause, play, prev, repeat, shuffle, speaker, Sprite};

use super::bars::paint_bar;
use super::palette::{rgb24, ACCENT, BG, MUTED, TEXT};

fn icon(fb: &mut PaintBuffer, r: &Rect, s: Sprite) {
    fb.blit_rgba8_scaled(r.x, r.y, r.w, r.h, &s.rgba, s.w, s.h);
}

pub fn paint_transport(fb: &mut PaintBuffer, v: &PlayerView, l: &Layout) {
    icon(fb, &l.shuffle, shuffle(48, rgb24(MUTED)));
    icon(fb, &l.prev, prev(48, rgb24(TEXT)));

    let disc = glow_disc(128, rgb24(ACCENT), rgb24(ACCENT));
    fb.blit_rgba8_scaled(l.play.x, l.play.y, l.play.w, l.play.h, &disc.rgba, disc.w, disc.h);
    let gs = l.play.w * 42 / 100;
    let gx = l.play.x + l.play.w.saturating_sub(gs) / 2;
    let gy = l.play.y + l.play.h.saturating_sub(gs) / 2;
    let g = if v.state == State::Playing { pause(64, rgb24(BG)) } else { play(64, rgb24(BG)) };
    fb.blit_rgba8_scaled(gx, gy, gs, gs, &g.rgba, g.w, g.h);

    icon(fb, &l.next, next(48, rgb24(TEXT)));
    icon(fb, &l.repeat, repeat(48, rgb24(MUTED)));

    let sp = speaker(48, rgb24(MUTED));
    fb.blit_rgba8_scaled(l.speaker.x, l.speaker.y, l.speaker.w, l.speaker.h, &sp.rgba, sp.w, sp.h);
    paint_bar(fb, &l.volume, v.volume_q15.max(0) as u32, 0x8000);
}
