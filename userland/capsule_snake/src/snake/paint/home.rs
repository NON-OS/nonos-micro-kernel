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
use nonos_toolkit::font::ttf::line_height;

use crate::snake::input::hover::{self, Tag};
use crate::snake::state::{Game, Phase};
use crate::snake::theme::{MUTED, TITLE};
use crate::snake::ui::button::{self, Style};
use crate::snake::ui::home_geom::{action, wordmark, ACTIONS, LABELS};
use crate::snake::ui::metrics::{PX_BODY, PX_WORDMARK, RADIUS_BTN};
use crate::snake::ui::text;

use super::{glow, home_cards};

const TAGLINE: &[u8] = b"Steer the line, take the light, stay alive";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    mark(fb, w, h);
    for index in 0..ACTIONS {
        let r = action(w, h, index);
        let lit = hover::is(Tag::HomeAction, index);
        button::paint(fb, r, LABELS[index], style(game, index), lit);
        if index == 0 {
            glow::bloom(fb, r, RADIUS_BTN);
        }
    }
    home_cards::paint(game, fb);
}

// Continue is only honest while a run is actually suspended behind the hub.
fn style(game: &Game, index: usize) -> Style {
    match index {
        0 => Style::Primary,
        1 if game.phase == Phase::Ready || game.phase == Phase::GameOver => Style::Disabled,
        _ => Style::Ghost,
    }
}

fn mark(fb: &mut PaintBuffer, w: u32, h: u32) {
    let r = wordmark(w, h);
    let title = b"SNAKE";
    let x = r.0 + r.2.saturating_sub(text::width_of(title, PX_WORDMARK)) / 2;
    text::left(fb, x, r.1, title, TITLE, PX_WORDMARK);
    let cut = text::fit(TAGLINE, PX_BODY, r.2);
    let tx = r.0 + r.2.saturating_sub(text::width_of(cut, PX_BODY)) / 2;
    text::left(fb, tx, r.1 + line_height(PX_WORDMARK).max(1) as u32, cut, MUTED, PX_BODY);
}
