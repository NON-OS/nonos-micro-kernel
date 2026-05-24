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

use super::theme::{WINDOW_HEIGHT, WINDOW_INITIAL_X, WINDOW_INITIAL_Y, WINDOW_WIDTH};

const WINDOW_ID: u32 = 0x4142_4F55;
const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;
const TITLE: &[u8] = b"About NONOS";

pub fn manifest() -> AppManifest {
    AppManifest {
        title: TITLE,
        window_id: WINDOW_ID,
        kind: WindowKind::Normal,
        initial_x: WINDOW_INITIAL_X,
        initial_y: WINDOW_INITIAL_Y,
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        input_kind_mask: INPUT_KEY_DOWN_BIT,
    }
}
