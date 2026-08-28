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

use capsule_text_editor_layout_tests::doc::style::{Family, RunStyle};

#[test]
fn body_is_regular_sans() {
    let s = RunStyle::body();
    assert!(!s.bold && !s.italic && !s.underline && !s.strike);
    assert_eq!(s.family, Family::Sans);
    assert_eq!(s.highlight, 0);
    assert_eq!(s.size_px, 16.0);
}

#[test]
fn headings_shrink_with_level_and_are_bold() {
    let h1 = RunStyle::heading(1);
    let h2 = RunStyle::heading(2);
    let h6 = RunStyle::heading(6);
    assert!(h1.bold && h2.bold && h6.bold);
    assert!(h1.size_px > h2.size_px);
    assert!(h2.size_px > h6.size_px);
    assert_eq!(h1.color, 0xFF17BED9);
}

#[test]
fn heading_level_is_clamped() {
    assert_eq!(RunStyle::heading(0).size_px, RunStyle::heading(1).size_px);
    assert_eq!(RunStyle::heading(99).size_px, RunStyle::heading(6).size_px);
}
