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
use nonos_toolkit::icons::draw;

use crate::snake::input::hover::{self, Tag};
use crate::snake::state::Game;
use crate::snake::theme::{MUTED, TITLE};
use crate::snake::ui::button::{self, Style};
use crate::snake::ui::card;
use crate::snake::ui::metrics::{GAP_TIGHT, ICON_MD, PAD, PX_HEAD, RADIUS_PANEL};
use crate::snake::ui::rect::Rect;
use crate::snake::ui::setup_geom::{head, head_h, panel, start, SECTIONS};
use crate::snake::ui::setup_geom_rows::HEADS;
use crate::snake::ui::text;
use crate::snake::ui::{icon_table, metrics};

use super::{setup_chips, setup_toggles};

const START: &[u8] = b"Start run";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    card::panel(fb, backing(panel(w, h)), RADIUS_PANEL);
    for section in 0..SECTIONS {
        band_head(fb, head(w, h, section), section);
    }
    setup_chips::paint(game, fb);
    setup_toggles::paint(game, fb);
    button::paint(fb, start(w, h), START, Style::Primary, hover::is(Tag::Start, 0));
}

// The panel geometry is the content column; the ground behind it is a painter
// decoration and is grown from the same rect rather than measured again.
fn backing(r: Rect) -> Rect {
    (r.0.saturating_sub(PAD), r.1.saturating_sub(PAD), r.2 + PAD * 2, r.3 + PAD * 2)
}

fn band_head(fb: &mut PaintBuffer, r: Rect, section: usize) {
    let mark = icon_table::option(section);
    let y = r.1 + head_h().saturating_sub(ICON_MD) / 2;
    draw(fb, mark, r.0, y, ICON_MD, MUTED);
    let x = r.0 + ICON_MD + GAP_TIGHT;
    let cut = text::fit(HEADS[section], PX_HEAD, r.2.saturating_sub(ICON_MD + metrics::GAP_TIGHT));
    text::left(fb, x, r.1, cut, TITLE, PX_HEAD);
}
