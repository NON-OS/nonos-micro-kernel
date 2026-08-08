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

use super::constants::{FOOTER_H, TEXT_LEFT};
use super::shade::elevate;
use crate::term::theme::{ACCENT, DIM};

/// The hint bar: what to press, and what it does.
///
/// Two colours rather than one. A single shade for both makes the bar read as
/// one long sentence, which is why a reader stops seeing it at all; the key
/// carries the accent so the eye can find it and skip the rest.
const HINTS: &[(&str, &str)] = &[
    ("Tab", "complete"),
    ("|", "pipe"),
    (">", "redirect"),
    ("^W", "word"),
    ("^K", "kill"),
    ("^L", "clear"),
];

/// Space between a key and its label, and between one pair and the next. The
/// pair gap is wider so the pairs group before they read left to right.
const KEY_GAP: i32 = 5;
const PAIR_GAP: i32 = 18;

pub fn draw_footer(fb: &mut PaintBuffer, bg: u32) {
    let y = fb.height.saturating_sub(FOOTER_H);
    fb.fill_rect(0, y, fb.width, FOOTER_H, elevate(bg, 10));
    // A hairline rather than a block in another shade. It separates the bar
    // from the body without spending a row of height to do it.
    fb.fill_rect(0, y, fb.width, 1, elevate(bg, 22));

    let baseline = (y + 3) as i32;
    let mut x = TEXT_LEFT as i32;
    for (key, label) in HINTS {
        let key_w = fb.measure_ttf(key, 12.0);
        let label_w = fb.measure_ttf(label, 12.0);
        // Stop before a pair would run under the right edge, rather than
        // clipping one in half.
        if x + key_w + KEY_GAP + label_w > fb.width as i32 - TEXT_LEFT as i32 {
            break;
        }
        let _ = fb.text_ttf(x, baseline, key, ACCENT, 12.0);
        x += key_w + KEY_GAP;
        let _ = fb.text_ttf(x, baseline, label, DIM, 12.0);
        x += label_w + PAIR_GAP;
    }
}
