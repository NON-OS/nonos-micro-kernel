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
use alloc::vec;
use alloc::vec::Vec;

use super::ast::{ClassItem, Re};
use super::parser::P;

impl<'a> P<'a> {
    pub fn parse_atom(&mut self) -> Re {
        match self.peek() {
            Some('(') => self.parse_group(),
            Some('[') => self.parse_class(),
            Some('.') => {
                self.bump();
                Re::Any
            }
            Some('^') => {
                self.bump();
                Re::Start
            }
            Some('$') => {
                self.bump();
                Re::End
            }
            Some('\\') => self.parse_escape(),
            Some(c) => {
                self.bump();
                Re::Char(c)
            }
            None => Re::Concat(Vec::new()),
        }
    }

    fn parse_group(&mut self) -> Re {
        self.bump();
        let mut idx = None;
        if self.eat('?') {
            self.eat(':');
        } else {
            self.groups += 1;
            idx = Some(self.groups);
        }
        let inner = self.parse_alt();
        self.eat(')');
        Re::Group(Box::new(inner), idx)
    }

    fn parse_escape(&mut self) -> Re {
        self.bump();
        match self.bump() {
            Some('d') => Re::Class(vec![ClassItem::Digit(false)], false),
            Some('D') => Re::Class(vec![ClassItem::Digit(true)], false),
            Some('w') => Re::Class(vec![ClassItem::Word(false)], false),
            Some('W') => Re::Class(vec![ClassItem::Word(true)], false),
            Some('s') => Re::Class(vec![ClassItem::Space(false)], false),
            Some('S') => Re::Class(vec![ClassItem::Space(true)], false),
            Some('b') => Re::WordB(true),
            Some('B') => Re::WordB(false),
            Some('n') => Re::Char('\n'),
            Some('t') => Re::Char('\t'),
            Some('r') => Re::Char('\r'),
            Some(c) => Re::Char(c),
            None => Re::Char('\\'),
        }
    }
}
