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

use crate::protocol::{Request, DAMAGE_COMMIT_REQ_LEN, E_INVAL};
use crate::server::respond;
use crate::state::{damage::Rect, Context};

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if body.len() != DAMAGE_COMMIT_REQ_LEN {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    let Some(x) = super::u32_at(body, 0) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(y) = super::u32_at(body, 4) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(width) = super::u32_at(body, 8) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(height) = super::u32_at(body, 12) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    if width == 0
        || height == 0
        || x.saturating_add(width) > ctx.width
        || y.saturating_add(height) > ctx.height
    {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    ctx.damage.accumulate(Rect { x, y, width, height });
    respond::status(sender_pid, req, 0, tx)
}
