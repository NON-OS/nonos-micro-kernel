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
use crate::constants::{MAX_SECTORS_PER_REQUEST, SECTOR_SIZE};
use crate::io::{submit, BlkError};
use crate::protocol::{
    read_u32_le, read_u64_le, Request, E_INVAL, E_IO, E_MSGSIZE, E_NXIO, RW_HEADER_LEN,
};
use crate::queue::Direction;
use crate::server::error::reply_with_status;
use crate::setup::Driver;
pub fn handle(driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < RW_HEADER_LEN {
        reply_with_status(tx, req, E_MSGSIZE);
        return;
    }
    let lba = match read_u64_le(body, 0) {
        Some(v) => v,
        None => {
            reply_with_status(tx, req, E_MSGSIZE);
            return;
        }
    };
    let nsectors = match read_u32_le(body, 8) {
        Some(v) => v,
        None => {
            reply_with_status(tx, req, E_MSGSIZE);
            return;
        }
    };
    if nsectors == 0 || nsectors > MAX_SECTORS_PER_REQUEST {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let bytes_n = (nsectors as usize) * SECTOR_SIZE;
    if body.len() != RW_HEADER_LEN + bytes_n || req.payload_len as usize != RW_HEADER_LEN + bytes_n
    {
        reply_with_status(tx, req, E_MSGSIZE);
        return;
    }
    let last = match lba.checked_add(nsectors as u64) {
        Some(v) => v,
        None => {
            reply_with_status(tx, req, E_INVAL);
            return;
        }
    };
    if last > driver.capacity_sectors {
        reply_with_status(tx, req, E_NXIO);
        return;
    }
    unsafe {
        let dst = driver.queue.data_mut(bytes_n as u32);
        dst.copy_from_slice(&body[RW_HEADER_LEN..RW_HEADER_LEN + bytes_n]);
    }
    let outcome =
        submit(driver.regs, &mut driver.queue, driver.irq_grant, Direction::Write, lba, nsectors);
    let status = match outcome {
        Ok(()) => 0,
        Err(BlkError::Unsupported) => E_INVAL,
        Err(_) => E_IO,
    };
    reply_with_status(tx, req, status);
}