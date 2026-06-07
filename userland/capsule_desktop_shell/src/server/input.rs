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

use nonos_libc::{INPUT_KIND_BUTTON_DOWN, INPUT_KIND_TOUCH};

use crate::protocol::{read_i32, read_u16, read_u32};
use crate::server::handlers::launcher_focus;
use crate::state::Context;

pub fn handle(ctx: &mut Context, buf: &[u8]) -> bool {
    if buf.len() < 40 || read_u32(buf, 0) != Some(0x4E49_4E50) {
        return false;
    }
    if read_u16(buf, 4) != Some(1) {
        return true;
    }
    let Some(kind) = read_u16(buf, 8) else {
        return true;
    };
    let Some(x) = read_i32(buf, 16) else {
        return true;
    };
    let Some(y) = read_i32(buf, 20) else {
        return true;
    };
    if x >= 0 && y >= 0 && matches!(kind, INPUT_KIND_TOUCH | INPUT_KIND_BUTTON_DOWN) {
        launcher_focus::handle(ctx, x as u32, y as u32);
    }
    true
}
