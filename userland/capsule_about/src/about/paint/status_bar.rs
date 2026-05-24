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

use crate::about::theme::{HINT, STATUS_BAR_HEIGHT, TAB_BAR, TEXT_LEFT};

const HINT_LINE: &[u8] = b"Tab/Shift-Tab cycle sections   Up/Down scroll line   PgUp/PgDn scroll page   Esc close";

pub fn paint(fb: &mut PaintBuffer) {
    let y = fb.height - STATUS_BAR_HEIGHT;
    fb.fill_rect(0, y, fb.width, STATUS_BAR_HEIGHT, TAB_BAR);
    fb.text(TEXT_LEFT, y + 6, HINT_LINE, HINT);
}
