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

use alloc::string::String;

use crate::browser::css::selector::Simple;

use super::attr_test::parse_attr;
use super::compound::parse_compound;
use super::pseudo::parse_pseudo_tail;

pub fn parse_simple(tok: &str) -> Simple {
    let mut s = Simple::empty();
    // Attribute tests split off first: tag.class[attr="v"]:pseudo.
    let mut plain = String::new();
    let mut rest = tok;
    while let Some(open) = rest.find('[') {
        plain.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let Some(close) = after.find(']') else {
            rest = "";
            break;
        };
        if s.attrs.len() < 4 {
            if let Some(t) = parse_attr(&after[..close]) {
                s.attrs.push(t);
            }
        }
        rest = after.get(close + 1..).unwrap_or("");
    }
    plain.push_str(rest);
    // The compound runs to the first colon; the rest is the pseudo tail.
    let mut pseudo_root = false;
    let tok = match plain.find(':') {
        Some(col) => {
            let tail = &plain[col..];
            if col == 0 && tail.trim_start_matches(':').starts_with("root") {
                pseudo_root = true;
            } else {
                parse_pseudo_tail(tail, &mut s.pseudo);
            }
            &plain[..col]
        }
        None => plain.as_str(),
    };
    parse_compound(tok, pseudo_root, &mut s);
    s
}
