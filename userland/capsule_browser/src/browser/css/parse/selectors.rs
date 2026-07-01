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

use crate::browser::css::selector::{Selector, Simple};

use super::simple::parse_simple;

pub fn parse_selectors(list: &str) -> Vec<Selector> {
    let mut out: Vec<Selector> = Vec::new();
    for part in list.split(',') {
        let mut simples: Vec<Simple> = part.split_whitespace().take(32).map(parse_simple).collect();
        let Some(key) = simples.pop() else {
            continue;
        };
        out.push(Selector { key, ancestors: simples });
        if out.len() >= 64 {
            break;
        }
    }
    out
}
