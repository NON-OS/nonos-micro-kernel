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

use crate::about::theme::{BODY, HEADLINE, LINE_HEIGHT, TEXT_LEFT};

pub const LABEL_X: u32 = TEXT_LEFT;
pub const VALUE_X: u32 = TEXT_LEFT + 152;

pub fn pair(label: &[u8], value: &[u8], y: u32, fb: &mut PaintBuffer) {
    fb.text(LABEL_X, y, label, BODY);
    fb.text(VALUE_X, y, value, HEADLINE);
}

pub fn single(text: &[u8], y: u32, fb: &mut PaintBuffer) {
    fb.text(TEXT_LEFT, y, text, BODY);
}

pub fn line_y(index: u32, top: u32) -> u32 {
    top + index * LINE_HEIGHT
}
