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

use crate::protocol::{Request, E_INVAL, SCENE_SUBMIT_REQ_LEN};
use crate::server::respond;
use crate::state::{damage::Rect, Context, Layer};

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if body.len() != SCENE_SUBMIT_REQ_LEN {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    let Some(surface_handle) = super::u64_at(body, 0) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(x) = super::u32_at(body, 8) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(y) = super::u32_at(body, 12) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(width) = super::u32_at(body, 16) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(height) = super::u32_at(body, 20) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(z) = super::u32_at(body, 24) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    // Clip a window that runs past the display edge rather than refusing it.
    // The WM's display size can differ from the compositor's by an edge pixel,
    // and a hard reject there left a window's first submit refused and blank.
    if width == 0 || height == 0 || x >= ctx.width || y >= ctx.height {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    let width = width.min(ctx.width - x);
    let height = height.min(ctx.height - y);
    // A window that reopened its surface (a resize, for one) re-submits with a
    // new handle for the same owner. The old handle's backing is gone, so drop
    // it from the attach cache now instead of letting it linger until the
    // reaper runs. Otherwise a reused handle number could serve a stale
    // mapping and the compositor would paint from freed memory.
    let stale = ctx
        .scene
        .layers()
        .find(|l| l.owner_pid == sender_pid && l.surface_handle != surface_handle)
        .map(|l| l.surface_handle);

    let layer = Layer {
        owner_pid: sender_pid,
        surface_handle,
        x,
        y,
        width,
        height,
        z,
        in_use: true,
        miss_count: 0,
    };
    if ctx.scene.submit(layer).is_err() {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    if let Some(old) = stale {
        let _ = ctx.attach.forget(old);
    }
    ctx.damage.accumulate(Rect { x, y, width, height });
    respond::status(sender_pid, req, 0, tx)
}
