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

use crate::compositor_client::push_damage_commit;
use crate::protocol::{Request, E_BUSY, E_INVAL, E_NOMEM, TRAY_LABEL_MAX, TRAY_REGISTER_REQ_LEN};
use crate::render::{menubar_rect, paint_chrome, sync_toast_layer};
use crate::server::respond;
use crate::state::{Context, TrayEntry};

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != TRAY_REGISTER_REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(tray_id) = super::u32_at(body, 0) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(label_len) = super::u32_at(body, 4) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if tray_id == 0 || label_len == 0 || label_len as usize > TRAY_LABEL_MAX {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let mut entry = TrayEntry {
        owner_pid: sender_pid,
        tray_id,
        label_len,
        label: [0; TRAY_LABEL_MAX],
        in_use: true,
    };
    let Some(label) = body.get(8..8 + label_len as usize) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    entry.label[..label_len as usize].copy_from_slice(label);
    if ctx.tray.find(sender_pid, tray_id).is_some() {
        let _ = respond::status(sender_pid, req, E_BUSY, tx);
        return;
    }
    if ctx.tray.insert(entry).is_err() {
        let _ = respond::status(sender_pid, req, E_NOMEM, tx);
        return;
    }
    paint_chrome(ctx);
    let r = menubar_rect(ctx.width);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, r.x, r.y, r.width, r.height);
    sync_toast_layer(ctx);
    let _ = respond::status(sender_pid, req, 0, tx);
}
