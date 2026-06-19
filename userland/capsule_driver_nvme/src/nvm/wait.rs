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

use core::ptr::read_volatile;

use super::constants::{COMPLETION_POLL_LIMIT, IO_ENTRIES};
use super::queue::IoQueue;
use crate::admin::Completion;
use crate::error::{NvmeError, NvmeResult};
use crate::regs::Regs;

impl IoQueue {
    pub(super) fn wait(&mut self, regs: Regs, cid: u16) -> NvmeResult<()> {
        for _ in 0..COMPLETION_POLL_LIMIT {
            let c = self.completion();
            if c.phase() == self.phase && c.cid == cid {
                self.advance(regs);
                return if c.successful() { Ok(()) } else { Err(NvmeError::AdminCommandFailed) };
            }
            core::hint::spin_loop();
        }
        Err(NvmeError::ControllerTimeout)
    }

    fn completion(&self) -> Completion {
        let slot =
            self.cq.user_va() + (self.cq_head as u64) * (core::mem::size_of::<Completion>() as u64);
        unsafe { read_volatile(slot as *const Completion) }
    }

    fn advance(&mut self, regs: Regs) {
        self.cq_head = (self.cq_head + 1) % IO_ENTRIES;
        if self.cq_head == 0 {
            self.phase = !self.phase;
        }
        unsafe { regs.w32(self.cq_db, self.cq_head as u32) };
    }
}
