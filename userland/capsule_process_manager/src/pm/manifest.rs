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

pub const WIDTH: u32 = 1240;
pub const HEIGHT: u32 = 780;

// Keys drive the table; buttons sort/select by click, the wheel scrolls, and the
// absolute pointer keeps the button coordinates current.
const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;
const INPUT_POINTER_ABS_BIT: u32 = 1 << 3;
const INPUT_WHEEL_BIT: u32 = 1 << 4;
const INPUT_BUTTON_DOWN_BIT: u32 = 1 << 5;

pub fn manifest() -> AppManifest {
    AppManifest {
        title: "NØNOS Processes".as_bytes(),
        window_id: 0x504D_4752,
        kind: WindowKind::Normal,
        initial_x: 340,
        initial_y: 210,
        width: WIDTH,
        height: HEIGHT,
        input_kind_mask: INPUT_KEY_DOWN_BIT
            | INPUT_BUTTON_DOWN_BIT
            | INPUT_WHEEL_BIT
            | INPUT_POINTER_ABS_BIT,
    }
}
