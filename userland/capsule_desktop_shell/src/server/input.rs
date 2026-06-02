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

use nonos_libc::{INPUT_KIND_BUTTON_DOWN, INPUT_KIND_POINTER_ABS};

use crate::compositor_client::push_damage_commit;
use crate::render::paint_chrome;
use crate::server::handlers::launcher_focus;
use crate::state::Context;

const CURSOR_SIZE: u32 = 12;

pub fn handle(ctx: &mut Context, buf: &[u8]) -> bool {
    if buf.len() < 40 || u32::from_le_bytes(buf[0..4].try_into().unwrap()) != 0x4E49_4E50 {
        return false;
    }
    if u16::from_le_bytes(buf[4..6].try_into().unwrap()) != 1 {
        return true;
    }
    let kind = u16::from_le_bytes(buf[8..10].try_into().unwrap());
    let x = i32::from_le_bytes(buf[16..20].try_into().unwrap());
    let y = i32::from_le_bytes(buf[20..24].try_into().unwrap());
    if x >= 0 && y >= 0 {
        if kind == INPUT_KIND_POINTER_ABS {
            repaint_cursor(ctx, x as u32, y as u32);
        } else if kind == INPUT_KIND_BUTTON_DOWN {
            launcher_focus::handle(ctx, x as u32, y as u32);
        }
    }
    true
}

fn repaint_cursor(ctx: &mut Context, x: u32, y: u32) {
    let old_x = ctx.pointer_x;
    let old_y = ctx.pointer_y;
    let had_cursor = ctx.pointer_visible;
    ctx.pointer_x = x.min(ctx.width.saturating_sub(1));
    ctx.pointer_y = y.min(ctx.height.saturating_sub(1));
    ctx.pointer_visible = true;
    paint_chrome(ctx);
    let left = if had_cursor { old_x.min(ctx.pointer_x) } else { ctx.pointer_x };
    let top = if had_cursor { old_y.min(ctx.pointer_y) } else { ctx.pointer_y };
    let right = if had_cursor { old_x.max(ctx.pointer_x) } else { ctx.pointer_x } + CURSOR_SIZE;
    let bottom = if had_cursor { old_y.max(ctx.pointer_y) } else { ctx.pointer_y } + CURSOR_SIZE;
    let width = right.saturating_sub(left).min(ctx.width.saturating_sub(left)).max(1);
    let height = bottom.saturating_sub(top).min(ctx.height.saturating_sub(top)).max(1);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, left, top, width, height);
}
