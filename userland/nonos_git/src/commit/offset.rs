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

//! The timezone offset on an author or committer line.
//!
//! Carried verbatim rather than normalised: it is hashed into the commit id,
//! so a commit re-encoded with a different offset is a different commit.

extern crate alloc;

use alloc::vec::Vec;

/// Append `+HHMM` or `-HHMM` for a count of minutes east of UTC.
pub(super) fn write(out: &mut Vec<u8>, offset_minutes: i16) {
    let (sign, abs) = if offset_minutes < 0 {
        (b'-', (-(offset_minutes as i32)) as u32)
    } else {
        (b'+', offset_minutes as u32)
    };
    out.push(sign);
    push_two(out, abs / 60);
    push_two(out, abs % 60);
}

/// Parse `+HHMM` or `-HHMM` into minutes east of UTC.
pub(super) fn parse(bytes: &[u8]) -> Option<i16> {
    if bytes.len() != 5 {
        return None;
    }
    let sign: i16 = match bytes[0] {
        b'+' => 1,
        b'-' => -1,
        _ => return None,
    };
    let hours = two_digits(&bytes[1..3])?;
    let minutes = two_digits(&bytes[3..5])?;
    if minutes >= 60 {
        return None;
    }
    Some(sign * (hours as i16 * 60 + minutes as i16))
}

fn push_two(out: &mut Vec<u8>, v: u32) {
    out.push(b'0' + ((v / 10) % 10) as u8);
    out.push(b'0' + (v % 10) as u8);
}

fn two_digits(b: &[u8]) -> Option<u8> {
    if !b[0].is_ascii_digit() || !b[1].is_ascii_digit() {
        return None;
    }
    Some((b[0] - b'0') * 10 + (b[1] - b'0'))
}
