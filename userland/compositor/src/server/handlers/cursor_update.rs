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

use crate::protocol::{Request, CURSOR_UPDATE_REQ_LEN, E_INVAL};
use crate::server::respond;
use crate::state::{damage::Rect, Context};

const CURSOR_SIDE: u32 = 32;

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if body.len() != CURSOR_UPDATE_REQ_LEN {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    let Some(x) = super::u32_at(body, 0) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(y) = super::u32_at(body, 4) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(visible_raw) = super::u32_at(body, 8) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let visible = visible_raw != 0;
    if x >= ctx.width || y >= ctx.height {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    let prev = ctx.cursor.update(x, y, visible);
    if prev.visible {
        let w = core::cmp::min(CURSOR_SIDE, ctx.width.saturating_sub(prev.x));
        let h = core::cmp::min(CURSOR_SIDE, ctx.height.saturating_sub(prev.y));
        ctx.damage.accumulate(Rect { x: prev.x, y: prev.y, width: w, height: h });
    }
    if visible {
        let w = core::cmp::min(CURSOR_SIDE, ctx.width.saturating_sub(x));
        let h = core::cmp::min(CURSOR_SIDE, ctx.height.saturating_sub(y));
        ctx.damage.accumulate(Rect { x, y, width: w, height: h });
    }
    respond::status(sender_pid, req, 0, tx)
}
