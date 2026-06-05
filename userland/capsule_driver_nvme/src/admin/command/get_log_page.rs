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
    pub const fn get_log_page(cid: u16, nsid: u32, lid: u8, bytes: u32, prp1: u64) -> Self {
        let dwords = bytes / 4;
        let numd = dwords - 1;
        Self {
            cdw0: 0x02 | ((cid as u32) << 16),
            nsid,
            cdw2: 0,
            cdw3: 0,
            mptr: 0,
            prp1,
            prp2: 0,
            cdw10: (lid as u32) | ((numd & 0xffff) << 16),
            cdw11: (numd >> 16) & 0xffff,
            cdw12: 0,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
        }
    }
}
