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

use super::constants::DATA_BYTES;
use super::prp::build_prp;
use super::queue::IoQueue;
use crate::admin::Submission;
use crate::error::{NvmeError, NvmeResult};
use crate::regs::Regs;

impl IoQueue {
    pub fn transfer(&mut self, regs: Regs, lba: u64, sectors: u32, write: bool) -> NvmeResult<()> {
        // Size the transfer by the formatted LBA size, and refuse anything that
        // would exceed the fixed DMA data buffer. Callers already bound sectors
        // by max_sectors(); this is the defensive floor against a bad count.
        let bytes = sectors as u64 * self.lba_size as u64;
        if bytes == 0 || bytes > DATA_BYTES {
            return Err(NvmeError::InvalidTransfer);
        }
        let (prp1, prp2) = build_prp(self, bytes);
        let cid = self.cid;
        self.cid = self.cid.wrapping_add(1).max(1);
        let nsid = self.nsid;
        let nlb = (sectors - 1) as u16;
        self.submit(regs, Submission::nvm_rw(cid, nsid, lba, nlb, prp1, prp2, write));
        self.wait(regs, cid)
    }
}
