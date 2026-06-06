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
use crate::transaction::{control, valid_lengths, TransferError, TransferRequest, TransferResult};

use super::run::run;

pub fn transfer(
    driver: &Driver,
    req: TransferRequest<'_>,
) -> Result<TransferResult, TransferError> {
    if req.addr > 0x7F || !valid_lengths(req.write.len(), req.read_len) {
        return Err(TransferError::Invalid);
    }
    let regs = driver.regs;
    control::wait_idle(regs)?;
    control::set_target(regs, req.addr)?;
    control::enable(regs)?;
    let result = run(regs, req);
    let _ = control::disable(regs);
    result
}
