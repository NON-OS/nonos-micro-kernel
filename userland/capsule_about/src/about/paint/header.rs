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

use crate::about::data::product::{NAME, TAGLINE};
use crate::about::theme::{HEADER, HEADER_HEIGHT, HEADLINE, TEXT_LEFT};

pub fn paint(fb: &mut PaintBuffer) {
    fb.fill_rect(0, 0, fb.width, HEADER_HEIGHT, HEADER);
    fb.text(TEXT_LEFT, 12, NAME, HEADLINE);
    fb.text(TEXT_LEFT + 64, 16, TAGLINE, HEADLINE);
}
