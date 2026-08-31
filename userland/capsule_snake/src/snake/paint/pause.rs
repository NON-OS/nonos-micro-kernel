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

use crate::snake::input::hover::{self, Tag};
use crate::snake::theme::TITLE;
use crate::snake::ui::button::{self, Style};
use crate::snake::ui::card;
use crate::snake::ui::metrics::{PX_TITLE, RADIUS_PANEL};
use crate::snake::ui::pause_geom::{action, modal, title, ACTIONS, LABELS};
use crate::snake::ui::text;

use super::glow;

const HEADING: &[u8] = b"Paused";

// The board keeps painting underneath, so everything here has to blend: the
// scrim, the shadow and the panel all land on pixels that already exist.
pub fn paint(fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    glow::scrim(fb);
    let m = modal(w, h);
    glow::shade(fb, m, RADIUS_PANEL);
    card::panel(fb, m, RADIUS_PANEL);
    let t = title(w, h);
    text::left(fb, t.0, t.1, HEADING, TITLE, PX_TITLE);
    for index in 0..ACTIONS {
        let lit = hover::is(Tag::PauseAction, index);
        button::paint(fb, action(w, h, index), LABELS[index], style(index), lit);
    }
}

fn style(index: usize) -> Style {
    match index {
        0 => Style::Primary,
        3 => Style::Danger,
        _ => Style::Ghost,
    }
}
