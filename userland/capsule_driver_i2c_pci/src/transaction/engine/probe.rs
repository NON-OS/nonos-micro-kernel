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
use crate::transaction::{TransferError, TransferRequest};

use super::transfer::transfer;

pub fn probe(driver: &Driver, addr: u8) -> Result<bool, TransferError> {
    let req = TransferRequest { addr, flags: 0, write: &[], read_len: 1 };
    match transfer(driver, req) {
        Ok(_) => Ok(true),
        Err(TransferError::Nack) => Ok(false),
        Err(e) => Err(e),
    }
}
