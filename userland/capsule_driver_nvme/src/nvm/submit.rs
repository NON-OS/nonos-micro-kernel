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

use super::constants::IO_ENTRIES;
use super::queue::IoQueue;
use crate::admin::Submission;
use crate::regs::Regs;

impl IoQueue {
    pub(super) fn submit(&mut self, regs: Regs, cmd: Submission) {
        let slot =
            self.sq.user_va() + (self.sq_tail as u64) * (core::mem::size_of::<Submission>() as u64);
        unsafe { write_volatile(slot as *mut Submission, cmd) };
        self.sq_tail = (self.sq_tail + 1) % IO_ENTRIES;
        unsafe { regs.w32(self.sq_db, self.sq_tail as u32) };
    }
}
