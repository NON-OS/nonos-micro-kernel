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

const CHEVRON: &[&[Pt]] = &[&[(350, 180), (700, 500), (350, 820)]];
const MAGNIFIER: &[&[Pt]] = &[&[(650, 650), (880, 880)]];
const GRID: &[&[Pt]] = &[
    &[(120, 120), (440, 120), (440, 440), (120, 440), (120, 120)],
    &[(560, 120), (880, 120), (880, 440), (560, 440), (560, 120)],
    &[(120, 560), (440, 560), (440, 880), (120, 880), (120, 560)],
    &[(560, 560), (880, 560), (880, 880), (560, 880), (560, 560)],
];
const LIST: &[&[Pt]] =
    &[&[(330, 250), (880, 250)], &[(330, 500), (880, 500)], &[(330, 750), (880, 750)]];
const SLIDERS: &[&[Pt]] = &[&[(120, 300), (880, 300)], &[(120, 700), (880, 700)]];
const PLUS: &[&[Pt]] = &[&[(500, 180), (500, 820)], &[(180, 500), (820, 500)]];
const BACK: &[&[Pt]] = &[&[(250, 500), (850, 500)], &[(480, 270), (250, 500), (480, 730)]];
const FORWARD: &[&[Pt]] = &[&[(150, 500), (750, 500)], &[(520, 270), (750, 500), (520, 730)]];
const CHECK: &[&[Pt]] = &[&[(170, 540), (420, 780), (840, 240)]];

/// The chrome glyphs. This is the last stage of the table, so an icon that
/// reaches it unmatched draws nothing rather than panicking mid-frame.
pub fn ui_glyph(i: Icon) -> Glyph {
    let polys: &'static [&'static [Pt]] = match i {
        Icon::Chevron => CHEVRON,
        Icon::Magnifier => MAGNIFIER,
        Icon::Grid => GRID,
        Icon::List => LIST,
        Icon::Sliders => SLIDERS,
        Icon::Plus => PLUS,
        Icon::Back => BACK,
        Icon::Forward => FORWARD,
        Icon::Check => CHECK,
        _ => &[],
    };
    Glyph { polys, rings: ui_rings(i) }
}

fn ui_rings(i: Icon) -> &'static [Ring] {
    match i {
        Icon::Magnifier => &[(430, 430, 300)],
        Icon::List => &[(150, 250, 60), (150, 500, 60), (150, 750, 60)],
        Icon::Sliders => &[(650, 300, 110), (350, 700, 110)],
        Icon::Ellipsis => &[(200, 500, 80), (500, 500, 80), (800, 500, 80)],
        _ => &[],
    }
}
