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

use crate::app::AppManifest;

const INPUT_POINTER_ABS_BIT: u32 = 1 << 3;
const INPUT_WHEEL_BIT: u32 = 1 << 4;
const INPUT_BUTTON_DOWN_BIT: u32 = 1 << 5;
const INPUT_BUTTON_UP_BIT: u32 = 1 << 6;
const INPUT_TOUCH_BIT: u32 = 1 << 7;

pub(super) fn input_mask(manifest: &AppManifest) -> u32 {
    manifest.input_kind_mask
        | INPUT_POINTER_ABS_BIT
        | INPUT_WHEEL_BIT
        | INPUT_BUTTON_DOWN_BIT
        | INPUT_BUTTON_UP_BIT
        | INPUT_TOUCH_BIT
}
