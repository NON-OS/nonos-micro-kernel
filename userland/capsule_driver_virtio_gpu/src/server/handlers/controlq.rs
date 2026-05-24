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
use crate::protocol::{Request, HDR_LEN, STATUS_LEN};
use crate::server::respond;
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let body = &mut tx[HDR_LEN + STATUS_LEN..];
    body[0..8].copy_from_slice(&driver.queue_grant.to_le_bytes());
    body[8..16].copy_from_slice(&driver.queue_user_va.to_le_bytes());
    body[16..24].copy_from_slice(&driver.queue_device_addr.to_le_bytes());
    respond::payload(sender_pid, req, crate::protocol::CONTROLQ_STATE_LEN, tx);
}