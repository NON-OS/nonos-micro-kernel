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

use super::action::Action;

pub const KEY_ESC: u32 = 0x1B;
pub const KEY_UP: u32 = 0x1201;
pub const KEY_DOWN: u32 = 0x1202;
pub const KEY_LEFT: u32 = 0x1203;
pub const KEY_RIGHT: u32 = 0x1204;

const SKIP_SECS: i32 = 10;
const VOLUME_STEP: i32 = 5;

pub fn from_key(code: u32) -> Action {
    match code {
        KEY_ESC => return Action::Close,
        KEY_UP => return Action::VolumeBy(VOLUME_STEP),
        KEY_DOWN => return Action::VolumeBy(-VOLUME_STEP),
        KEY_LEFT => return Action::SeekBy(-SKIP_SECS),
        KEY_RIGHT => return Action::SeekBy(SKIP_SECS),
        _ => {}
    }
    if code > 0x7F {
        return Action::None;
    }
    match code as u8 {
        b' ' => Action::TogglePlay,
        b'm' | b'M' => Action::ToggleMute,
        b'0' => Action::Restart,
        _ => Action::None,
    }
}
