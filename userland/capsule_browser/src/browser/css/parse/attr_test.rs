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

use alloc::string::{String, ToString};

use crate::browser::css::selector::AttrTest;

// One [attr] body into a named test: presence, or the =, *=, ^=, $=, ~=, |=
// operators with an optionally quoted value.
pub(super) fn parse_attr(body: &str) -> Option<(String, AttrTest)> {
    let Some(eq) = body.find('=') else {
        let name = body.trim().to_ascii_lowercase();
        if name.is_empty() {
            return None;
        }
        return Some((name, AttrTest::Present));
    };
    let (mut name_part, op) = match body.as_bytes().get(eq.wrapping_sub(1)) {
        Some(b'*') => (&body[..eq - 1], b'*'),
        Some(b'^') => (&body[..eq - 1], b'^'),
        Some(b'$') => (&body[..eq - 1], b'$'),
        Some(b'~') => (&body[..eq - 1], b'~'),
        Some(b'|') => (&body[..eq - 1], b'|'),
        _ => (&body[..eq], b'='),
    };
    name_part = name_part.trim();
    let name = name_part.to_ascii_lowercase();
    if name.is_empty() {
        return None;
    }
    let val = body[eq + 1..].trim().trim_matches('"').trim_matches('\'').to_string();
    let test = match op {
        b'*' => AttrTest::Contains(val),
        b'^' => AttrTest::Starts(val),
        b'$' => AttrTest::Ends(val),
        b'~' => AttrTest::Word(val),
        b'|' => AttrTest::Lang(val),
        _ => AttrTest::Eq(val),
    };
    Some((name, test))
}
