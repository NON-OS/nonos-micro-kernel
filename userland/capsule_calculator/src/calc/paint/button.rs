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

use crate::calc::buttons::{Button, Role};
use crate::calc::theme::{
    BUTTON_BORDER, EQUALS_BG, EQUALS_TEXT, FUNCTION_BG, FUNCTION_TEXT, MEMORY_BG, MEMORY_TEXT,
    NUMBER_BG, NUMBER_TEXT, OPERATOR_BG, OPERATOR_TEXT,
};

const CELL_WIDTH: u32 = 8;
const CELL_HEIGHT: u32 = 8;

pub fn paint(fb: &mut PaintBuffer, btn: &Button, x: u32, y: u32, w: u32, h: u32) {
    let (bg, fg) = match btn.role {
        Role::Number => (NUMBER_BG, NUMBER_TEXT),
        Role::Operator => (OPERATOR_BG, OPERATOR_TEXT),
        Role::Equals => (EQUALS_BG, EQUALS_TEXT),
        Role::Function => (FUNCTION_BG, FUNCTION_TEXT),
        Role::Memory => (MEMORY_BG, MEMORY_TEXT),
    };
    fb.fill_rect(x, y, w, h, bg);
    fb.fill_rect(x, y, w, 1, BUTTON_BORDER);
    fb.fill_rect(x, y + h - 1, w, 1, BUTTON_BORDER);
    let label_w = (btn.label.len() as u32) * CELL_WIDTH;
    let tx = x + (w.saturating_sub(label_w)) / 2;
    let ty = y + (h.saturating_sub(CELL_HEIGHT)) / 2;
    fb.text(tx, ty, btn.label, fg);
}
