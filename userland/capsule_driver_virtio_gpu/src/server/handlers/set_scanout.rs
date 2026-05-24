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

use crate::constants::{VG_FORMAT_B8G8R8A8_UNORM, VG_MAX_SCANOUTS};
use crate::device::cmd::{self, transfer_to_host_2d::Rect};
use crate::driver::Driver;
use crate::protocol::{le_u32, Request, E_BUSY, E_DEVICE, E_INVAL, SET_SCANOUT_REQ_LEN};
use crate::server::respond;
use crate::state::Scanout;

pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some((scanout_id, resource_id, rect)) = decode(body) else {
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
    if res.format != VG_FORMAT_B8G8R8A8_UNORM {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let fence_id = driver.fences.issue();
    if cmd::set_scanout(&driver.control_queue, fence_id, scanout_id, resource_id, rect).is_err() {
        respond::status(sender_pid, req, E_DEVICE, tx);
        return;
    }
    driver.scanouts.record(
        scanout_id,
        Scanout {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
            current_resource_id: resource_id,
            enabled: true,
        },
    );
    respond::status(sender_pid, req, 0, tx);
}

fn decode(body: &[u8]) -> Option<(u32, u32, Rect)> {
    if body.len() != SET_SCANOUT_REQ_LEN {
        return None;
    }
    let scanout_id = le_u32(body, 0)?;
    let resource_id = le_u32(body, 4)?;
    let x = le_u32(body, 8)?;
    let y = le_u32(body, 12)?;
    let width = le_u32(body, 16)?;
    let height = le_u32(body, 20)?;
    if (scanout_id as usize) >= VG_MAX_SCANOUTS || width == 0 || height == 0 {
        return None;
    }
    Some((scanout_id, resource_id, Rect { x, y, width, height }))
}
