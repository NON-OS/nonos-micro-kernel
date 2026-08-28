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

use nonos_app_skeleton::{AppManifest, WindowKind};

use super::grid::{WIN_H, WIN_W};

const WINDOW_ID: u32 = 0x534E_414B;
const TITLE: &[u8] = b"Snake";
const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;
const INPUT_POINTER_ABS_BIT: u32 = 1 << 3;
const INPUT_BUTTON_DOWN_BIT: u32 = 1 << 5;
const INPUT_MASK: u32 = INPUT_KEY_DOWN_BIT | INPUT_POINTER_ABS_BIT | INPUT_BUTTON_DOWN_BIT;

pub fn manifest() -> AppManifest {
    AppManifest {
        title: TITLE,
        window_id: WINDOW_ID,
        kind: WindowKind::Normal,
        initial_x: 120,
        initial_y: 70,
        width: WIN_W,
        height: WIN_H,
        input_kind_mask: INPUT_MASK,
    }
}
