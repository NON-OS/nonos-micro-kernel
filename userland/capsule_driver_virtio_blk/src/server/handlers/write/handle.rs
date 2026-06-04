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
use super::request::write_request;
use crate::io::{submit, BlkError};
use crate::protocol::{Request, E_INVAL, E_IO, RW_HEADER_LEN};
use crate::queue::Direction;
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle(driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    let parsed = match write_request(driver, req, body) {
        Ok(v) => v,
        Err(status) => {
            reply_with_status(tx, req, status);
            return;
        }
    };
    unsafe {
        let dst = driver.queue.data_mut(parsed.bytes_n as u32);
        dst.copy_from_slice(&body[RW_HEADER_LEN..RW_HEADER_LEN + parsed.bytes_n]);
    }
    let outcome = submit(
        driver.regs,
        &mut driver.queue,
        driver.irq_grant,
        Direction::Write,
        parsed.lba,
        parsed.nsectors,
    );
    let status = match outcome {
        Ok(()) => 0,
        Err(BlkError::Unsupported) => E_INVAL,
        Err(_) => E_IO,
    };
    reply_with_status(tx, req, status);
}
