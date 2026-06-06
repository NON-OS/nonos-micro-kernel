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

use super::constants::CELL_WIDTH;
use crate::term::theme::{ACCENT, FOREGROUND};

pub fn fetch_row(fb: &mut PaintBuffer, x: u32, y: u32, label: &[u8], value: &[u8]) {
    fb.text(x, y, label, ACCENT);
    fb.text(x + 9 * CELL_WIDTH, y, value, FOREGROUND);
}
