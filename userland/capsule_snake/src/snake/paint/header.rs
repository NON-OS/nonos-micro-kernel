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

use crate::snake::grid::{BOARD_X, TITLEBAR_H};
use crate::snake::state::Game;

const LABEL: u32 = 0xFF9A_A4B2;
const VALUE: u32 = 0xFF6A_D47A;

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let mut buf = [0u8; 10];
    let digits = itoa(game.score, &mut buf);
    fb.text_scaled(BOARD_X, TITLEBAR_H + 10, b"SCORE", LABEL, 2);
    fb.text_scaled(BOARD_X + 104, TITLEBAR_H + 10, digits, VALUE, 2);
}

fn itoa(mut value: u32, buf: &mut [u8; 10]) -> &[u8] {
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    &buf[i..]
}
