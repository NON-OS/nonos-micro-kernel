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
use crate::snake::state::{Game, Phase};
use crate::snake::ui::button::{self, Style};
use crate::snake::ui::play_geom_rows::{foot, FOOT_BTNS, FOOT_LABELS};

const RESUME: &[u8] = b"Resume";

// Sound has no mixer route from a windowed app, so it is drawn as the disabled
// control it is rather than as a button that quietly does nothing.
pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    for index in 0..FOOT_BTNS {
        let lit = hover::is(Tag::Foot, index);
        button::paint(fb, foot(w, h, index), label(game, index), style(index), lit);
    }
}

fn label(game: &Game, index: usize) -> &'static [u8] {
    if index == 0 && game.phase == Phase::Paused {
        return RESUME;
    }
    FOOT_LABELS[index]
}

fn style(index: usize) -> Style {
    match index {
        2 => Style::Disabled,
        3 => Style::Danger,
        _ => Style::Ghost,
    }
}
