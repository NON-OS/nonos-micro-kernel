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

/// The smallest fixed-format sense buffer that still carries the ASC/ASCQ pair
/// (bytes 12 and 13), which is what callers actually branch on.
pub const SENSE_MIN_LEN: usize = 14;

/// The decoded reason for a CHECK CONDITION. `sense_key` is the coarse class
/// (for example 0x02 NOT READY, 0x06 UNIT ATTENTION after a medium change) and
/// the additional-sense code pair narrows it down.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sense {
    pub sense_key: u8,
    pub asc: u8,
    pub ascq: u8,
}

/// Decode fixed-format sense data (response codes 0x70 current and 0x71
/// deferred). Descriptor-format sense (0x72/0x73) is not decoded here and
/// returns None, as does a buffer too short to hold the ASC/ASCQ pair.
pub fn parse_sense(raw: &[u8]) -> Option<Sense> {
    if raw.len() < SENSE_MIN_LEN {
        return None;
    }
    let response_code = raw[0] & 0x7f;
    if response_code != 0x70 && response_code != 0x71 {
        return None;
    }
    Some(Sense {
        sense_key: raw[2] & 0x0f,
        asc: raw[12],
        ascq: raw[13],
    })
}
