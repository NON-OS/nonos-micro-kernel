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
use crate::io::{submit, BlkError};
use crate::protocol::{Request, E_INVAL, E_IO};
use crate::queue::Direction;
use crate::server::error::reply_with_status;
use crate::setup::Driver;
pub fn handle(driver: &mut Driver, req: &Request, tx: &mut [u8]) {
    let outcome = submit(driver.regs, &mut driver.queue, driver.irq_grant, Direction::Flush, 0, 0);
    let status = match outcome {
        Ok(()) => 0,
        Err(BlkError::Unsupported) => E_INVAL,
        Err(_) => E_IO,
    };
    reply_with_status(tx, req, status);
}