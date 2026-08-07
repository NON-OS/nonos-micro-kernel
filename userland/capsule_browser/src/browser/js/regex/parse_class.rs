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

use super::ast::{ClassItem, Re};
use super::parser::P;

impl<'a> P<'a> {
    // `[...]` or `[^...]`, supporting shorthand escapes and `a-z` ranges.
    pub fn parse_class(&mut self) -> Re {
        self.bump();
        let neg = self.eat('^');
        let mut items = Vec::new();
        while let Some(c) = self.peek() {
            if c == ']' {
                break;
            }
            if c == '\\' {
                self.bump();
                items.push(self.class_escape());
                continue;
            }
            self.bump();
            let start = c;
            let dash_range =
                self.peek() == Some('-') && self.cs.get(self.i + 1).map_or(false, |&n| n != ']');
            if dash_range {
                self.bump();
                let end = self.bump().unwrap_or(start);
                items.push(ClassItem::Range(start, end));
            } else {
                items.push(ClassItem::Ch(start));
            }
        }
        self.eat(']');
        Re::Class(items, neg)
    }

    fn class_escape(&mut self) -> ClassItem {
        match self.bump() {
            Some('d') => ClassItem::Digit(false),
            Some('D') => ClassItem::Digit(true),
            Some('w') => ClassItem::Word(false),
            Some('W') => ClassItem::Word(true),
            Some('s') => ClassItem::Space(false),
            Some('S') => ClassItem::Space(true),
            Some('n') => ClassItem::Ch('\n'),
            Some('t') => ClassItem::Ch('\t'),
            Some('r') => ClassItem::Ch('\r'),
            Some(c) => ClassItem::Ch(c),
            None => ClassItem::Ch('\\'),
        }
    }
}
