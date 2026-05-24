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
use crate::device::cmd::{self, transfer_to_host_2d::Rect};
use crate::driver::Driver;
use crate::protocol::{le_u32, Request, E_BUSY, E_DEVICE, E_INVAL, FLUSH_REQ_LEN};
use crate::server::respond;
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != FLUSH_REQ_LEN {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(resource_id) = le_u32(body, 0) else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(x) = le_u32(body, 4) else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(y) = le_u32(body, 8) else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(w) = le_u32(body, 12) else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(h) = le_u32(body, 16) else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(res) = driver.resources.lookup(resource_id) else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if res.owner_pid != sender_pid {
        respond::status(sender_pid, req, E_BUSY, tx);
        return;
    }
    if w == 0 || h == 0 || x.saturating_add(w) > res.width || y.saturating_add(h) > res.height {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let fence_id = driver.fences.issue();
    if cmd::resource_flush(
        &driver.control_queue,
        fence_id,
        resource_id,
        Rect { x, y, width: w, height: h },
    )
    .is_err()
    {
        respond::status(sender_pid, req, E_DEVICE, tx);
        return;
    }
    respond::status(sender_pid, req, 0, tx);
}