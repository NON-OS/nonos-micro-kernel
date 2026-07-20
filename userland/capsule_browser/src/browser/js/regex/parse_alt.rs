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

use alloc::vec;
use alloc::vec::Vec;

use super::ast::Re;
use super::parser::P;

impl<'a> P<'a> {
    // alternation: concat ( '|' concat )*
    pub fn parse_alt(&mut self) -> Re {
        let mut alts = vec![self.parse_concat()];
        while self.eat('|') {
            alts.push(self.parse_concat());
        }
        if alts.len() == 1 {
            alts.pop().unwrap_or(Re::Concat(Vec::new()))
        } else {
            Re::Alt(alts)
        }
    }

    // concatenation: quantified atoms until `|`, `)`, or end of input.
    pub fn parse_concat(&mut self) -> Re {
        let mut parts = Vec::new();
        while let Some(c) = self.peek() {
            if c == '|' || c == ')' {
                break;
            }
            parts.push(self.parse_quant());
        }
        if parts.len() == 1 {
            parts.pop().unwrap_or(Re::Concat(Vec::new()))
        } else {
            Re::Concat(parts)
        }
    }
}
