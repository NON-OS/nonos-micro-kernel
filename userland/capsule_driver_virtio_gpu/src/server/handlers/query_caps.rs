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
use crate::driver::Driver;
use crate::protocol::{Request, HDR_LEN, QUERY_CAPS_RESP_LEN, STATUS_LEN};
use crate::server::respond;
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let (events_read, num_scanouts, num_capsets) = driver.config();
    let body_off = HDR_LEN + STATUS_LEN;
    tx[body_off..body_off + 4].copy_from_slice(&num_scanouts.to_le_bytes());
    tx[body_off + 4..body_off + 8].copy_from_slice(&num_capsets.to_le_bytes());
    tx[body_off + 8..body_off + 12].copy_from_slice(&events_read.to_le_bytes());
    // 3D capability: nonzero when VirGL is negotiated and render context 1 is
    // live, so a client knows SUBMIT_3D-backed surfaces are available.
    let virgl = u32::from(driver.virgl_ready);
    tx[body_off + 12..body_off + 16].copy_from_slice(&virgl.to_le_bytes());
    respond::payload(sender_pid, req, QUERY_CAPS_RESP_LEN, tx);
}
