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

use crate::snake::state::Game;
use crate::snake::theme::{LABEL, MUTED};
use crate::snake::ui::icon_table;
use crate::snake::ui::metrics::{GAP_TIGHT, ICON_MD, PX_BODY, ROW_H};
use crate::snake::ui::setup_geom_rows::{toggle, toggle_row, TOGGLES, TOGGLE_LABELS};
use crate::snake::ui::text;
use crate::snake::ui::toggle as switch;

// Wrap is not a free choice in every mode: Zen forces it on and Classic refuses
// it, so the switch is drawn locked rather than pretending to be live.
pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    let state = [game.opts.obstacles, game.opts.wraps(game.mode), game.opts.powerups];
    let locked = [false, game.mode.forces_wrap() || game.mode.hard_walls(), false];
    for index in 0..TOGGLES {
        let row = toggle_row(w, h, index);
        let mark_y = row.1 + ROW_H.saturating_sub(ICON_MD) / 2;
        draw(fb, icon_table::option(index), row.0, mark_y, ICON_MD, MUTED);
        let x = row.0 + ICON_MD + GAP_TIGHT;
        let top = text::centred_top(row.1, row.3, PX_BODY);
        text::left(fb, x, top, TOGGLE_LABELS[index], LABEL, PX_BODY);
        switch::paint(fb, toggle(w, h, index), state[index], locked[index]);
    }
}
