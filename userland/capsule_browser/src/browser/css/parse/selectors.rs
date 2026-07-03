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
use alloc::vec::Vec;

use crate::browser::css::selector::{Selector, Simple, Step};

use super::simple::parse_simple;

pub fn parse_selectors(list: &str) -> Vec<Selector> {
    let mut out: Vec<Selector> = Vec::new();
    for part in list.split(',') {
        // Give `>` its own token whether or not it was spaced.
        let spaced: String = part.chars().fold(String::new(), |mut s, ch| {
            if ch == '>' {
                s.push(' ');
                s.push('>');
                s.push(' ');
            } else {
                s.push(ch);
            }
            s
        });
        let mut simples: Vec<Simple> = Vec::new();
        let mut direct: Vec<bool> = Vec::new();
        let mut next_direct = false;
        for tok in spaced.split_whitespace().take(32) {
            if tok == ">" {
                next_direct = true;
                continue;
            }
            // Sibling combinators are approximated as descendant so the rule
            // still applies rather than dying on a bogus tag.
            if tok == "+" || tok == "~" {
                continue;
            }
            simples.push(parse_simple(tok));
            direct.push(next_direct);
            next_direct = false;
        }
        let Some(key) = simples.pop() else {
            continue;
        };
        // direct[i] says step i is a direct child of step i-1; matching
        // walks outward from the key, so the flag moves onto the ancestor.
        let key_direct = direct.pop().unwrap_or(false);
        let mut ancestors: Vec<Step> = Vec::new();
        let mut below = key_direct;
        for (simple, d) in simples.into_iter().zip(direct.into_iter()).rev() {
            ancestors.push(Step { simple, direct: below });
            below = d;
        }
        out.push(Selector { key, ancestors });
        if out.len() >= 64 {
            break;
        }
    }
    out
}
