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
use nonos_libc::mk_ipc_send;
use crate::constants::{MAX_SECTORS_PER_REQUEST, SECTOR_SIZE};
use crate::io::{submit, BlkError};
use crate::protocol::{
    encode_response_header, read_u32_le, read_u64_le, write_status, Request, E_INVAL, E_IO,
    E_MSGSIZE, E_NXIO, KERNEL_REPLY_ENDPOINT, READ_REQ_LEN, RESP_HDR_LEN, STATUS_LEN,
};
use crate::queue::Direction;
use crate::server::error::reply_with_status;
use crate::setup::Driver;
pub fn handle(driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < READ_REQ_LEN || req.payload_len != READ_REQ_LEN as u32 {
        reply_with_status(tx, req, E_MSGSIZE);
        return;
    }
    let lba = match read_u64_le(body, 0) {
        Some(v) => v,
        None => { reply_with_status(tx, req, E_MSGSIZE); return; }
    };
    let nsectors = match read_u32_le(body, 8) {
        Some(v) => v,
        None => { reply_with_status(tx, req, E_MSGSIZE); return; }
    };
    if nsectors == 0 || nsectors > MAX_SECTORS_PER_REQUEST {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let last = match lba.checked_add(nsectors as u64) {
        Some(v) => v,
        None => { reply_with_status(tx, req, E_INVAL); return; }
    };
    if last > driver.capacity_sectors {
        reply_with_status(tx, req, E_NXIO);
        return;
    }
    let outcome = submit(driver.regs, &mut driver.queue, driver.irq_grant, Direction::Read, lba, nsectors);
    match outcome {
        Ok(()) => {}
        Err(BlkError::Unsupported) => {
            reply_with_status(tx, req, E_INVAL);
            return;
        }
        Err(_) => {
            reply_with_status(tx, req, E_IO);
            return;
        }
    }
    let bytes_n = (nsectors as usize) * SECTOR_SIZE;
    let payload_len = STATUS_LEN as u32 + bytes_n as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let buf = unsafe { driver.queue.data(bytes_n as u32) };
    tx[RESP_HDR_LEN + STATUS_LEN..RESP_HDR_LEN + STATUS_LEN + bytes_n].copy_from_slice(buf);
    if mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + STATUS_LEN + bytes_n) < 0 {
        return;
    }
}
