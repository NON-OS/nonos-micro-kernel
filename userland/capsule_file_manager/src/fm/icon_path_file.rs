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

use super::icon_path::{Glyph, Icon, Pt};
use super::icon_path_ui::ui_glyph;

const DOC: &[&[Pt]] = &[
    &[(250, 90), (620, 90), (780, 260), (780, 910), (250, 910), (250, 90)],
    &[(620, 90), (620, 260), (780, 260)],
];
const FOLDER: &[&[Pt]] =
    &[&[(90, 820), (90, 220), (400, 220), (490, 340), (910, 340), (910, 820), (90, 820)]];
const CUBE: &[&[Pt]] = &[
    &[(500, 100), (880, 310), (500, 520), (120, 310), (500, 100)],
    &[(120, 310), (120, 700), (500, 910), (880, 700), (880, 310)],
    &[(500, 520), (500, 910)],
];
const ARCHIVE: &[&[Pt]] = &[
    &[(100, 180), (900, 180), (900, 380), (100, 380), (100, 180)],
    &[(170, 380), (170, 860), (830, 860), (830, 380)],
    &[(400, 570), (600, 570)],
];
const TRASH: &[&[Pt]] = &[
    &[(120, 260), (880, 260)],
    &[(390, 260), (390, 130), (610, 130), (610, 260)],
    &[(200, 260), (250, 880), (750, 880), (800, 260)],
    &[(420, 430), (420, 740)],
    &[(580, 430), (580, 740)],
];

/// The content glyphs. Unmatched icons continue on to `icon_path_ui`.
pub fn file_glyph(i: Icon) -> Glyph {
    let polys: &'static [&'static [Pt]] = match i {
        Icon::Doc => DOC,
        Icon::Folder => FOLDER,
        Icon::Cube => CUBE,
        Icon::Archive => ARCHIVE,
        Icon::Trash => TRASH,
        _ => return ui_glyph(i),
    };
    Glyph { polys, rings: &[] }
}
