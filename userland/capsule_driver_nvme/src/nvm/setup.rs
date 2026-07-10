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

use super::constants::{IO_ENTRIES, IO_QID};
use super::queue::IoQueue;
use crate::admin::AdminQueue;
use crate::error::NvmeResult;
use crate::regs::Regs;

pub fn bring_up(
    device_id: u64,
    epoch: u64,
    regs: Regs,
    stride: u8,
    admin: &mut AdminQueue,
    nsid: u32,
    capacity_sectors: u64,
    lba_size: u32,
) -> NvmeResult<IoQueue> {
    let io =
        IoQueue::allocate(device_id, epoch, stride, IO_QID, nsid, capacity_sectors, lba_size)?;
    admin.create_io_cq(regs, stride, IO_QID, IO_ENTRIES, io.cq.device_addr())?;
    admin.create_io_sq(regs, stride, IO_QID, IO_QID, IO_ENTRIES, io.sq.device_addr())?;
    Ok(io)
}
