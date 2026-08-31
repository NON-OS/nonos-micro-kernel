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

use crate::snake::state::run::MAX_RUNS;
use crate::snake::state::RunRecord;

use super::codec;
use super::mode_code::mode_of;

pub fn runs(bytes: &[u8]) -> Result<Vec<RunRecord>, &'static str> {
    let count = codec::count_of(bytes, codec::MAGIC_RUNS, codec::RUN_LEN, MAX_RUNS)?;
    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        let at = codec::HEADER_LEN + index * codec::RUN_LEN;
        let entry = &bytes[at..at + codec::RUN_LEN];
        let score = u32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]);
        let seq = u32::from_le_bytes([entry[4], entry[5], entry[6], entry[7]]);
        let length = u16::from_le_bytes([entry[8], entry[9]]);
        out.push(RunRecord::new(score, mode_of(entry[10])?, length, seq));
    }
    Ok(out)
}

pub fn awards(bytes: &[u8]) -> Result<Vec<u16>, &'static str> {
    let count = codec::count_of(bytes, codec::MAGIC_AWARDS, codec::AWARD_LEN, codec::MAX_AWARDS)?;
    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        let at = codec::HEADER_LEN + index * codec::AWARD_LEN;
        out.push(u16::from_le_bytes([bytes[at], bytes[at + 1]]));
    }
    Ok(out)
}
