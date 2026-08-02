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

use super::offset;

/// An author or committer line's contents.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Signature {
    pub name: String,
    pub email: String,
    /// Seconds since the epoch.
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
        offset::write(out, self.offset_minutes);
    }

    /// Parse the part of an `author` or `committer` line after the field name.
    ///
    /// The name is taken up to the last `<`, so a name that itself contains one
    /// still parses the way git reads it.
    pub(super) fn parse(line: &[u8]) -> Option<Signature> {
        let lt = line.iter().rposition(|b| *b == b'<')?;
        let gt = line.iter().rposition(|b| *b == b'>')?;
        if gt < lt || lt == 0 {
            return None;
        }
        let name = core::str::from_utf8(&line[..lt - 1]).ok()?;
        let email = core::str::from_utf8(&line[lt + 1..gt]).ok()?;

        let rest = &line[gt + 1..];
        let rest = if rest.first() == Some(&b' ') { &rest[1..] } else { rest };
        let space = rest.iter().position(|b| *b == b' ')?;
        let when = parse_decimal(&rest[..space])?;
        let offset_minutes = offset::parse(&rest[space + 1..])?;

        Some(Signature {
            name: String::from(name),
            email: String::from(email),
            when,
            offset_minutes,
        })
    }
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
