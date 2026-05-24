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
use crate::device::cmd;
use crate::driver::Driver;
use crate::protocol::{le_u32, le_u64, Request, ATTACH_BACKING_REQ_LEN, E_BUSY, E_DEVICE, E_INVAL};
use crate::server::respond;
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != ATTACH_BACKING_REQ_LEN {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(resource_id) = le_u32(body, 0) else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(backing_addr) = le_u64(body, 8) else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(backing_len) = le_u64(body, 16) else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if backing_addr == 0 || backing_len == 0 || backing_len > u32::MAX as u64 {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(existing) = driver.resources.lookup(resource_id) else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if existing.owner_pid != sender_pid {
        respond::status(sender_pid, req, E_BUSY, tx);
        return;
    }
    let fence_id = driver.fences.issue();
    if cmd::attach_backing(
        &driver.control_queue,
        fence_id,
        resource_id,
        backing_addr,
        backing_len as u32,
    )
    .is_err()
    {
        respond::status(sender_pid, req, E_DEVICE, tx);
        return;
    }
    driver.resources.update(resource_id, |r| {
        r.backing_addr = backing_addr;
        r.backing_len = backing_len as u32;
    });
    respond::status(sender_pid, req, 0, tx);
}