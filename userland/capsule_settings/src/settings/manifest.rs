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

pub const WIDTH: u32 = 760;
pub const HEIGHT: u32 = 520;

const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;

pub fn manifest() -> AppManifest {
    AppManifest {
        title: "NØNOS Settings".as_bytes(),
        window_id: 0x5345_5447,
        kind: WindowKind::Normal,
        initial_x: 420,
        initial_y: 44,
        width: WIDTH,
        height: HEIGHT,
        input_kind_mask: INPUT_KEY_DOWN_BIT,
    }
}
