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
use crate::constants::{IC_DATA_CMD_READ, IC_DATA_CMD_RESTART};
use crate::transaction::{TransferRequest, FLAG_RESTART_ON_READ};

pub fn read_cmd(req: &TransferRequest<'_>, ci: usize) -> u32 {
    let first_read_after_write = ci == req.write.len() && !req.write.is_empty();
    if first_read_after_write && req.flags & FLAG_RESTART_ON_READ != 0 {
        IC_DATA_CMD_READ | IC_DATA_CMD_RESTART
    } else {
        IC_DATA_CMD_READ
    }
}
