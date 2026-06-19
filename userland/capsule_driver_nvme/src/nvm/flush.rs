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

use super::queue::IoQueue;
use crate::admin::Submission;
use crate::error::NvmeResult;
use crate::regs::Regs;

impl IoQueue {
    pub fn flush(&mut self, regs: Regs) -> NvmeResult<()> {
        let cid = self.cid;
        self.cid = self.cid.wrapping_add(1).max(1);
        let nsid = self.nsid;
        self.submit(regs, Submission::nvm_flush(cid, nsid));
        self.wait(regs, cid)
    }
}
