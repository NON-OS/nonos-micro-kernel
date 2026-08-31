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

pub const ELLIPSIS: &str = "..";

pub fn trim<'a>(fb: &PaintBuffer, text: &'a str, budget: i32, px: f32) -> (&'a str, bool) {
    if budget <= 0 {
        return ("", false);
    }
    if fb.measure_ttf(text, px) <= budget {
        return (text, false);
    }
    let room = budget - fb.measure_ttf(ELLIPSIS, px);
    let mut cut = 0;
    for (i, _) in text.char_indices() {
        if fb.measure_ttf(&text[..i], px) > room {
            break;
        }
        cut = i;
    }
    (&text[..cut], true)
}
