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

//! Reading a signature line.

extern crate alloc;

use alloc::string::String;

use crate::commit::offset;

use super::decimal::parse as parse_decimal;
use super::types::Signature;

impl Signature {
    /// The name runs to the last `<`, so a name containing one still parses
    /// the way git reads it.
    pub(in crate::commit) fn parse(line: &[u8]) -> Option<Signature> {
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

        Some(Signature {
            name: String::from(name),
            email: String::from(email),
            when: parse_decimal(&rest[..space])?,
            offset_minutes: offset::parse(&rest[space + 1..])?,
        })
    }
}
