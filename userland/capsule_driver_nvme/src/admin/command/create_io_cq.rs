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

use super::submission::Submission;

impl Submission {
    pub const fn create_io_cq(cid: u16, qid: u16, qsize: u16, cq_phys: u64) -> Self {
        Self {
            cdw0: 0x05 | ((cid as u32) << 16),
            nsid: 0,
            cdw2: 0,
            cdw3: 0,
            mptr: 0,
            prp1: cq_phys,
            prp2: 0,
            cdw10: (((qsize - 1) as u32) << 16) | qid as u32,
            cdw11: 1,
            cdw12: 0,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
        }
    }
}
