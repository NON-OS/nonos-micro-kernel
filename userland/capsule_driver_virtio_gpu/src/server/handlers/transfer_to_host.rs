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
use crate::protocol::{
    le_u32, le_u64, Request, E_BUSY, E_DEVICE, E_INVAL, TRANSFER_TO_HOST_REQ_LEN,
};
use crate::server::respond;

pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some((resource_id, rect, offset)) = decode(body) else {
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
    let x_end = rect.x.saturating_add(rect.width);
    let y_end = rect.y.saturating_add(rect.height);
    if rect.width == 0 || rect.height == 0 || x_end > res.width || y_end > res.height {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let fence_id = driver.fences.issue();
    if cmd::transfer_to_host_2d(&driver.control_queue, fence_id, resource_id, rect, offset).is_err()
    {
        respond::status(sender_pid, req, E_DEVICE, tx);
        return;
    }
    respond::status(sender_pid, req, 0, tx);
}

fn decode(body: &[u8]) -> Option<(u32, Rect, u64)> {
    if body.len() != TRANSFER_TO_HOST_REQ_LEN {
        return None;
    }
    let resource_id = le_u32(body, 0)?;
    let x = le_u32(body, 4)?;
    let y = le_u32(body, 8)?;
    let width = le_u32(body, 12)?;
    let height = le_u32(body, 16)?;
    let offset = le_u64(body, 24)?;
    Some((resource_id, Rect { x, y, width, height }, offset))
}
