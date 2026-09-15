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

use super::icon_path::{Glyph, Icon, Pt, Ring};
use super::icon_path_file::file_glyph;

const HOME: &[&[Pt]] = &[
    &[(100, 520), (500, 150), (900, 520)],
    &[(200, 470), (200, 880), (800, 880), (800, 470)],
    &[(410, 880), (410, 650), (590, 650), (590, 880)],
];
const CLOCK: &[&[Pt]] = &[&[(500, 270), (500, 520), (700, 620)]];
const STAR: &[&[Pt]] = &[&[
    (500, 100), (610, 390), (920, 390), (670, 580), (760, 880), (500, 700),
    (240, 880), (330, 580), (80, 390), (390, 390), (500, 100),
]];
const PEOPLE: &[&[Pt]] = &[
    &[(160, 850), (190, 670), (330, 580), (510, 580), (650, 670), (680, 850)],
    &[(720, 590), (830, 610), (900, 700), (915, 850)],
];
const TAG: &[&[Pt]] =
    &[&[(120, 120), (520, 120), (900, 500), (500, 880), (120, 500), (120, 120)]];
const DOWNLOAD: &[&[Pt]] = &[
    &[(500, 130), (500, 640)],
    &[(330, 470), (500, 650), (670, 470)],
    &[(160, 720), (160, 870), (840, 870), (840, 720)],
];

/// Entry point for the whole table: the place-list glyphs resolve here, every
/// other one falls through to `icon_path_file` and then `icon_path_ui`.
pub fn glyph(i: Icon) -> Glyph {
    let polys: &'static [&'static [Pt]] = match i {
        Icon::Home => HOME,
        Icon::Clock => CLOCK,
        Icon::Star => STAR,
        Icon::People => PEOPLE,
        Icon::Tag => TAG,
        Icon::Download => DOWNLOAD,
        _ => return file_glyph(i),
    };
    Glyph { polys, rings: rings_for(i) }
}

fn rings_for(i: Icon) -> &'static [Ring] {
    match i {
        Icon::Clock => &[(500, 500, 400)],
        Icon::People => &[(420, 320, 180), (760, 360, 130)],
        Icon::Tag => &[(280, 290, 80)],
        _ => &[],
    }
}
