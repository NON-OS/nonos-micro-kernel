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
    pub const fn nvm_rw(
        cid: u16,
        nsid: u32,
        slba: u64,
        nlb: u16,
        prp1: u64,
        prp2: u64,
        write: bool,
    ) -> Self {
        let opcode: u32 = if write { 0x01 } else { 0x02 };
        Self {
            cdw0: opcode | ((cid as u32) << 16),
            nsid,
            cdw2: 0,
            cdw3: 0,
            mptr: 0,
            prp1,
            prp2,
            cdw10: slba as u32,
            cdw11: (slba >> 32) as u32,
            cdw12: nlb as u32,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
        }
    }
}
