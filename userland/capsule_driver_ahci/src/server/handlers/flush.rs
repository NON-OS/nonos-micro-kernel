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

use crate::engine::flush;
use crate::protocol::{Request, E_IO, E_NODEV};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle(driver: &mut Driver, req: &Request, tx: &mut [u8]) {
    let regs = driver.regs;
    let port = match driver.block.as_mut() {
        Some(p) => p,
        None => return reply_with_status(tx, req, E_NODEV),
    };
    let status = if flush(port, regs).is_ok() { 0 } else { E_IO };
    reply_with_status(tx, req, status);
}
