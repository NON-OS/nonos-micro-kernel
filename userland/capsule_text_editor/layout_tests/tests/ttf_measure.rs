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

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};

fn advance(bytes: &[u8], text: &str, px: f32) -> f32 {
    let f = FontRef::try_from_slice(bytes).unwrap();
    let sf = f.as_scaled(PxScale::from(px));
    text.chars().map(|c| sf.h_advance(f.glyph_id(c))).sum()
}

fn regular() -> Vec<u8> {
    let p = concat!(env!("CARGO_MANIFEST_DIR"), "/../../toolkit/assets/fonts/NotoSans-Regular.ttf");
    std::fs::read(p).unwrap()
}

#[test]
fn small_sizes_are_not_clamped_to_min_ui_px() {
    let b = regular();
    let at_12 = advance(&b, "hello", 12.0);
    let at_17 = advance(&b, "hello", 17.0);
    assert!(at_12 < at_17, "12px must measure narrower than 17px");
}

#[test]
fn advance_scales_linearly_with_size() {
    let b = regular();
    let a = advance(&b, "hello world", 12.0);
    let c = advance(&b, "hello world", 24.0);
    assert!((c - 2.0 * a).abs() < 0.01, "{c} should be 2x {a}");
}

#[test]
fn the_empty_string_has_zero_advance() {
    assert_eq!(advance(&regular(), "", 16.0), 0.0);
}
