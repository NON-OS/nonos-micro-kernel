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
use crate::constants::{IC_DATA_CMD, TIMEOUT_ITERS};
use crate::regs::Regs;
use crate::transaction::{TransferError, TransferRequest, TransferResult};

use super::check_abort::check_abort;
use super::done::done;
use super::drain_rx::drain_rx;
use super::read_cmd::read_cmd;
use super::rx_space::rx_space;
use super::take_write::take_write;
use super::tx_space::tx_space;

pub fn run(regs: Regs, req: TransferRequest<'_>) -> Result<TransferResult, TransferError> {
    let mut out = TransferResult::empty();
    let total = req.write.len() + req.read_len;
    let (mut wi, mut ri, mut ci) = (0usize, 0usize, 0usize);
    for _ in 0..TIMEOUT_ITERS {
        check_abort(regs, &mut out)?;
        drain_rx(regs, &mut out, &mut ri, req.read_len);
        while ci < total && tx_space(regs) > 0 && rx_space(regs) > 0 {
            let last = ci == total - 1;
            let mut cmd = if ci < req.write.len() {
                take_write(req.write, &mut wi)
            } else {
                read_cmd(&req, ci)
            };
            if last {
                cmd |= crate::constants::IC_DATA_CMD_STOP;
            }
            regs.write32(IC_DATA_CMD, cmd);
            ci += 1;
        }
        if ci >= total && ri >= req.read_len && done(regs) {
            out.read_len = ri;
            return Ok(out);
        }
        core::hint::spin_loop();
    }
    Err(TransferError::Timeout)
}
