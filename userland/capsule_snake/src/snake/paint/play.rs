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

use crate::snake::state::{Game, Phase};
use crate::snake::theme::{ACCENT, ACCENT_BORDER, ACCENT_TINT};
use crate::snake::ui::metrics::{BTN_H, GAP_WIDE, PX_BODY, RADIUS_PILL};
use crate::snake::ui::play_geom::{board, stage};
use crate::snake::ui::rect;
use crate::snake::ui::text;

use super::{board as board_paint, play_foot, play_hud, play_rail};

const READY: &[u8] = b"Press an arrow key to start";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    play_hud::paint(game, fb);
    board_paint::paint(game, fb, &board(w, h));
    play_rail::paint(game, fb);
    play_foot::paint(game, fb);
    if game.phase == Phase::Ready {
        ready(fb, w, h);
    }
}

// A pill low on the board rather than a full overlay: the snake spawns at the
// board centre, so the hint clears it and the player sees the line to steer.
fn ready(fb: &mut PaintBuffer, w: u32, h: u32) {
    let width = text::width_of(READY, PX_BODY) + BTN_H;
    let b = board(w, h);
    let r = rect::centred(stage(w, h), width, BTN_H);
    let y = b.y + b.h.saturating_sub(BTN_H + GAP_WIDE);
    fb.fill_round(r.0, y, r.2, r.3, RADIUS_PILL, ACCENT_TINT);
    fb.stroke_round(r.0, y, r.2, r.3, RADIUS_PILL, 1, ACCENT_BORDER);
    let x = r.0 + r.2.saturating_sub(text::width_of(READY, PX_BODY)) / 2;
    text::left(fb, x, text::centred_top(y, r.3, PX_BODY), READY, ACCENT, PX_BODY);
}
