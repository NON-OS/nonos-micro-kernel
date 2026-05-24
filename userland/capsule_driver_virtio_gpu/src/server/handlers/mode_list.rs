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
use crate::constants::VG_MAX_SCANOUTS;
use crate::driver::Driver;
use crate::protocol::{Request, HDR_LEN, MODE_LIST_ENTRY_LEN, STATUS_LEN};
use crate::server::respond;
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let body_off = HDR_LEN + STATUS_LEN;
    let mut emitted = 0usize;
    for sid in 0..VG_MAX_SCANOUTS as u32 {
        let Some(s) = driver.scanouts.get(sid) else { break };
        let off = body_off + emitted * MODE_LIST_ENTRY_LEN;
        tx[off..off + 4].copy_from_slice(&sid.to_le_bytes());
        tx[off + 4..off + 8].copy_from_slice(&(s.enabled as u32).to_le_bytes());
        tx[off + 8..off + 12].copy_from_slice(&s.width.to_le_bytes());
        tx[off + 12..off + 16].copy_from_slice(&s.height.to_le_bytes());
        tx[off + 16..off + 20].copy_from_slice(&s.x.to_le_bytes());
        tx[off + 20..off + 24].copy_from_slice(&s.y.to_le_bytes());
        tx[off + 24..off + 28].copy_from_slice(&s.current_resource_id.to_le_bytes());
        tx[off + 28..off + 32].fill(0);
        emitted += 1;
    }
    respond::payload(sender_pid, req, emitted * MODE_LIST_ENTRY_LEN, tx);
}