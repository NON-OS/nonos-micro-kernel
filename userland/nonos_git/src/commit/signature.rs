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

//! Who made a commit and when: `Name <email> <seconds> <offset>`.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// An author or committer line's contents.
///
/// The timestamp is seconds since the epoch and the offset is the author's
/// local zone as git writes it, `+HHMM` or `-HHMM`. Both are carried verbatim
/// rather than normalised, because they are hashed into the commit id: a
/// commit re-encoded with a different offset is a different commit.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub when: u64,
    /// Minutes east of UTC. `+0100` is 60, `-0500` is -300.
    pub offset_minutes: i16,
}

impl Signature {
    /// Append `Name <email> seconds ±HHMM` as git writes it.
    pub(super) fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.name.as_bytes());
        out.extend_from_slice(b" <");
        out.extend_from_slice(self.email.as_bytes());
        out.extend_from_slice(b"> ");
        push_decimal(out, self.when);
        out.push(b' ');
        self.write_offset(out);
    }

    fn write_offset(&self, out: &mut Vec<u8>) {
        let (sign, abs) = if self.offset_minutes < 0 {
            (b'-', (-(self.offset_minutes as i32)) as u32)
        } else {
            (b'+', self.offset_minutes as u32)
        };
        out.push(sign);
        let hours = abs / 60;
        let minutes = abs % 60;
        push_two(out, hours);
        push_two(out, minutes);
    }

    /// Parse the part of an `author`/`committer` line after the field name.
    pub(super) fn parse(line: &[u8]) -> Option<Signature> {
        // Name runs up to the last ` <`, so a name may itself contain `<`.
        let lt = rfind(line, b'<')?;
        let gt = rfind(line, b'>')?;
        if gt < lt || lt == 0 {
            return None;
        }
        let name = core::str::from_utf8(&line[..lt - 1]).ok()?;
        let email = core::str::from_utf8(&line[lt + 1..gt]).ok()?;

        let rest = &line[gt + 1..];
        let rest = if rest.first() == Some(&b' ') { &rest[1..] } else { rest };
        let space = rest.iter().position(|b| *b == b' ')?;
        let when = parse_decimal(&rest[..space])?;
        let offset_minutes = parse_offset(&rest[space + 1..])?;

        Some(Signature {
            name: String::from(name),
            email: String::from(email),
            when,
            offset_minutes,
        })
    }
}

/// Parse `+HHMM` or `-HHMM` into minutes east of UTC.
fn parse_offset(bytes: &[u8]) -> Option<i16> {
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

fn two_digits(b: &[u8]) -> Option<u8> {
    if !b[0].is_ascii_digit() || !b[1].is_ascii_digit() {
        return None;
    }
    Some((b[0] - b'0') * 10 + (b[1] - b'0'))
}

fn push_two(out: &mut Vec<u8>, v: u32) {
    out.push(b'0' + ((v / 10) % 10) as u8);
    out.push(b'0' + (v % 10) as u8);
}

fn push_decimal(out: &mut Vec<u8>, mut v: u64) {
    if v == 0 {
        out.push(b'0');
        return;
    }
    let start = out.len();
    while v > 0 {
        out.push(b'0' + (v % 10) as u8);
        v /= 10;
    }
    out[start..].reverse();
}

fn parse_decimal(bytes: &[u8]) -> Option<u64> {
    if bytes.is_empty() {
        return None;
    }
    let mut v: u64 = 0;
    for b in bytes {
        if !b.is_ascii_digit() {
            return None;
        }
        v = v.checked_mul(10)?.checked_add((b - b'0') as u64)?;
    }
    Some(v)
}

fn rfind(data: &[u8], byte: u8) -> Option<usize> {
    data.iter().rposition(|b| *b == byte)
}
