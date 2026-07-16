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

use core::ptr::write_volatile;

use super::constants::ADMIN_ENTRIES;
use super::sq0_tail::sq0_tail;
use super::types::AdminQueue;
use crate::admin::Submission;
use crate::regs::Regs;

impl AdminQueue {
    pub(super) fn submit(&mut self, regs: Regs, stride: u8, cmd: Submission) {
        let slot =
            self.sq.user_va() + (self.tail as u64) * (core::mem::size_of::<Submission>() as u64);
        unsafe { write_volatile(slot as *mut Submission, cmd) };
        self.tail = (self.tail + 1) % ADMIN_ENTRIES;
        // The controller DMA-reads the submission entry only after it sees the
        // new tail in the doorbell. Order the entry store ahead of the doorbell
        // store so the device never fetches a stale or half-written command.
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        unsafe { regs.w32(sq0_tail(stride), self.tail as u32) };
    }
}
