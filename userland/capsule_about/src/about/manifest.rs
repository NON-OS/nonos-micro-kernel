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

use super::ui::metrics::{WIN_H, WIN_W, WIN_X, WIN_Y};

const WINDOW_ID: u32 = 0x4142_4F55;

// Keys drive the sidebar and the scroll; the buttons select a section by click,
// and the absolute pointer keeps those coordinates current.
const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;
const INPUT_POINTER_ABS_BIT: u32 = 1 << 3;
const INPUT_BUTTON_DOWN_BIT: u32 = 1 << 5;

pub fn manifest() -> AppManifest {
    AppManifest {
        title: "NØNOS About".as_bytes(),
        window_id: WINDOW_ID,
        kind: WindowKind::Normal,
        initial_x: WIN_X,
        initial_y: WIN_Y,
        width: WIN_W,
        height: WIN_H,
        input_kind_mask: INPUT_KEY_DOWN_BIT | INPUT_BUTTON_DOWN_BIT | INPUT_POINTER_ABS_BIT,
    }
}
