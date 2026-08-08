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

pub fn cert_time_value(tag: u8, value: &[u8]) -> Option<u64> {
    match tag {
        0x17 if value.len() == 13 && value[12] == b'Z' => utc(value),
        0x18 if value.len() == 15 && value[14] == b'Z' => generalized(value),
        _ => None,
    }
}

fn utc(value: &[u8]) -> Option<u64> {
    let yy = digits(&value[0..2])? as u16;
    let year = if yy < 50 { 2000 + yy } else { 1900 + yy };
    stamp(
        year,
        digits(&value[2..4])?,
        digits(&value[4..6])?,
        digits(&value[6..8])?,
        digits(&value[8..10])?,
        digits(&value[10..12])?,
    )
}

fn generalized(value: &[u8]) -> Option<u64> {
    stamp(
        digits(&value[0..4])? as u16,
        digits(&value[4..6])?,
        digits(&value[6..8])?,
        digits(&value[8..10])?,
        digits(&value[10..12])?,
        digits(&value[12..14])?,
    )
}

fn digits(bytes: &[u8]) -> Option<u64> {
    let mut out = 0u64;
    for b in bytes {
        if !b.is_ascii_digit() {
            return None;
        }
        out = out * 10 + (*b - b'0') as u64;
    }
    Some(out)
}

fn stamp(year: u16, mon: u64, day: u64, hour: u64, min: u64, sec: u64) -> Option<u64> {
    if mon == 0 || mon > 12 || day == 0 || day > 31 || hour > 23 || min > 59 || sec > 60 {
        return None;
    }
    Some(
        year as u64 * 10_000_000_000
            + mon * 100_000_000
            + day * 1_000_000
            + hour * 10_000
            + min * 100
            + sec,
    )
}
