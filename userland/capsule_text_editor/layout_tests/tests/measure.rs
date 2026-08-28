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

use capsule_text_editor_layout_tests::doc::measure::{FixedMeasurer, Measurer};
use capsule_text_editor_layout_tests::doc::style::RunStyle;

#[test]
fn fixed_measurer_is_linear_in_length() {
    let m = FixedMeasurer;
    let s = RunStyle::body();
    assert_eq!(m.advance("", &s), 0.0);
    assert!(m.advance("aa", &s) > m.advance("a", &s));
    assert_eq!(m.advance("aa", &s), 2.0 * m.advance("a", &s));
}

#[test]
fn fixed_measurer_scales_with_size() {
    let m = FixedMeasurer;
    let small = RunStyle { size_px: 12.0, ..RunStyle::body() };
    let large = RunStyle { size_px: 24.0, ..RunStyle::body() };
    assert_eq!(m.advance("hello", &large), 2.0 * m.advance("hello", &small));
}

#[test]
fn line_height_exceeds_size() {
    let m = FixedMeasurer;
    let s = RunStyle::body();
    assert!(m.line_height(&s) > s.size_px);
    assert!(m.ascent(&s) < m.line_height(&s));
}
