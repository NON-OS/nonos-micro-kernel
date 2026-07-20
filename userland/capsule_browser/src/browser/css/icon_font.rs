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

// Webfont icon families draw glyphs from private-use codepoints or ligature
// names. We do not load their faces, so their ligature text (Material Icons
// spell "arrow_forward") would otherwise render as the literal word. A family
// is recognised here and its common ligatures mapped to a Unicode glyph that
// the built-in face can draw, so the icon reads as a symbol.

// The lowercased first family name names an icon font we cannot render.
pub(crate) fn is_icon_family(name: &str) -> bool {
    name.starts_with("material icons")
        || name.starts_with("material symbols")
        || name.starts_with("font awesome")
        || name == "fontawesome"
        || name == "material-icons"
        || name == "material-symbols"
}

// A Unicode glyph for a Material Icons/Symbols ligature name. The built-in text
// face only carries a handful of symbol glyphs (single guillemets, the times
// sign and the ellipsis; arrows, stars and gears all render as the missing-
// glyph box), so only names that map onto one of those are given a symbol. A
// forward-sense icon reads as a right guillemet, a back-sense one as a left,
// which is the common case (a "learn more" chevron, a carousel arrow).
// Unmapped names return None; the caller then hides the icon rather than
// showing its literal ligature word or a box.
pub fn map_ligature(name: &str) -> Option<&'static str> {
    let g = match name {
        "arrow_forward"
        | "arrow_right_alt"
        | "east"
        | "trending_flat"
        | "chevron_right"
        | "navigate_next"
        | "keyboard_arrow_right"
        | "play_arrow"
        | "double_arrow" => "\u{203A}",
        "arrow_back" | "west" | "chevron_left" | "navigate_before" | "keyboard_arrow_left" => {
            "\u{2039}"
        }
        "close" | "clear" | "cancel" | "delete" | "remove" => "\u{00D7}",
        "more_horiz" => "\u{2026}",
        _ => return None,
    };
    Some(g)
}
