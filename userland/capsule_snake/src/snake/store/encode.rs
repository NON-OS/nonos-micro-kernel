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

use alloc::vec::Vec;

use crate::snake::state::RunRecord;

use super::codec;
use super::mode_code::mode_byte;

// Run entry, 12 bytes: score u32 | seq u32 | length u16 | mode u8 | pad u8.
pub fn runs(list: &[RunRecord]) -> Vec<u8> {
    let mut out = Vec::with_capacity(codec::HEADER_LEN + list.len() * codec::RUN_LEN);
    codec::header(codec::MAGIC_RUNS, list.len(), &mut out);
    for run in list {
        out.extend_from_slice(&run.score.to_le_bytes());
        out.extend_from_slice(&run.seq.to_le_bytes());
        out.extend_from_slice(&run.length.to_le_bytes());
        out.push(mode_byte(run.mode));
        out.push(0);
    }
    out
}

// Award entry, 2 bytes: id u16.
pub fn awards(list: &[u16]) -> Vec<u8> {
    let mut out = Vec::with_capacity(codec::HEADER_LEN + list.len() * codec::AWARD_LEN);
    codec::header(codec::MAGIC_AWARDS, list.len(), &mut out);
    for id in list {
        out.extend_from_slice(&id.to_le_bytes());
    }
    out
}
