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

use nonos_libc::INPUT_KIND_BUTTON_DOWN;

use crate::server::handlers::launcher_focus;
use crate::state::Context;

pub fn handle(ctx: &mut Context, buf: &[u8]) -> bool {
    if buf.len() < 40 || u32::from_le_bytes(buf[0..4].try_into().unwrap()) != 0x4E49_4E50 {
        return false;
    }
    if u16::from_le_bytes(buf[4..6].try_into().unwrap()) != 1 {
        return true;
    }
    if u16::from_le_bytes(buf[8..10].try_into().unwrap()) != INPUT_KIND_BUTTON_DOWN {
        return true;
    }
    let x = i32::from_le_bytes(buf[16..20].try_into().unwrap());
    let y = i32::from_le_bytes(buf[20..24].try_into().unwrap());
    if x >= 0 && y >= 0 {
        launcher_focus::handle(ctx, x as u32, y as u32);
    }
    true
}
