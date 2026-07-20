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

// Recursive-descent regex parser state: the pattern characters, a cursor, and
// a running count of capturing groups seen so far.
pub struct P<'a> {
    pub cs: &'a [char],
    pub i: usize,
    pub groups: usize,
}

impl<'a> P<'a> {
    pub fn new(cs: &'a [char]) -> Self {
        P { cs, i: 0, groups: 0 }
    }
    pub fn peek(&self) -> Option<char> {
        self.cs.get(self.i).copied()
    }
    pub fn bump(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.i += 1;
        }
        c
    }
    pub fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.i += 1;
            true
        } else {
            false
        }
    }
    pub fn parse_int(&mut self) -> Option<usize> {
        let start = self.i;
        let mut v = 0usize;
        while let Some(c) = self.peek() {
            if let Some(d) = c.to_digit(10) {
                v = v.saturating_mul(10).saturating_add(d as usize);
                self.bump();
            } else {
                break;
            }
        }
        if self.i == start {
            None
        } else {
            Some(v)
        }
    }
}
