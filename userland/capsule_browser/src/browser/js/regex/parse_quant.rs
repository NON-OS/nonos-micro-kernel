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

use alloc::boxed::Box;

use super::ast::Re;
use super::parser::P;

impl<'a> P<'a> {
    // An atom followed by an optional quantifier (`*`, `+`, `?`, `{n,m}`) and a
    // lazy `?` marker.
    pub fn parse_quant(&mut self) -> Re {
        let atom = self.parse_atom();
        let bounds = match self.peek() {
            Some('*') => {
                self.bump();
                Some((0, None))
            }
            Some('+') => {
                self.bump();
                Some((1, None))
            }
            Some('?') => {
                self.bump();
                Some((0, Some(1)))
            }
            Some('{') => self.parse_brace(),
            _ => None,
        };
        match bounds {
            Some((min, max)) => {
                let greedy = !self.eat('?');
                Re::Repeat(Box::new(atom), min, max, greedy)
            }
            None => atom,
        }
    }

    // `{n}`, `{n,}`, or `{n,m}`. On any malformed input the cursor is restored
    // and None is returned so `{` is treated as a literal.
    fn parse_brace(&mut self) -> Option<(usize, Option<usize>)> {
        let save = self.i;
        self.bump();
        let min = match self.parse_int() {
            Some(n) => n,
            None => {
                self.i = save;
                return None;
            }
        };
        let max = if self.eat(',') {
            if self.peek() == Some('}') {
                None
            } else {
                match self.parse_int() {
                    Some(m) => Some(m),
                    None => {
                        self.i = save;
                        return None;
                    }
                }
            }
        } else {
            Some(min)
        };
        if !self.eat('}') {
            self.i = save;
            return None;
        }
        Some((min, max))
    }
}
