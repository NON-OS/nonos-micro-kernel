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

use crate::protocol::{Request, E_IO, E_MSGSIZE, E_NODEV, RW_HEADER_LEN};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle(driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    let regs = driver.regs;
    let io = match driver.io.as_mut() {
        Some(i) => i,
        None => return reply_with_status(tx, req, E_NODEV),
    };
    let (lba, nsectors) = match super::rw_parse::parse(body, io.capacity_sectors, io.max_sectors())
    {
        Ok(v) => v,
        Err(s) => return reply_with_status(tx, req, s),
    };
    let bytes = nsectors as usize * io.lba_size as usize;
    if body.len() != RW_HEADER_LEN + bytes || req.payload_len as usize != RW_HEADER_LEN + bytes {
        return reply_with_status(tx, req, E_MSGSIZE);
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            body[RW_HEADER_LEN..].as_ptr(),
            io.data.user_va() as *mut u8,
            bytes,
        );
    }
    let status = if io.transfer(regs, lba, nsectors, true).is_ok() { 0 } else { E_IO };
    reply_with_status(tx, req, status);
}
