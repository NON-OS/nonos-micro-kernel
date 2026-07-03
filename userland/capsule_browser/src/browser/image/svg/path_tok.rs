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

// Tokenizer over path data: command letters, numbers and arc flags,
// separated by whitespace or commas. Number scanning lives in path_num.rs.
pub(super) struct Tok<'a> {
    pub(super) s: &'a str,
    pub(super) i: usize,
}

impl<'a> Tok<'a> {
    pub fn new(s: &'a str) -> Self {
        Tok { s, i: 0 }
    }

    pub(super) fn skip_sep(&mut self) {
        let b = self.s.as_bytes();
        while self.i < b.len() && (b[self.i].is_ascii_whitespace() || b[self.i] == b',') {
            self.i += 1;
        }
    }

    // A command letter, if one is next.
    pub fn cmd(&mut self) -> Option<u8> {
        self.skip_sep();
        let c = *self.s.as_bytes().get(self.i)?;
        if c.is_ascii_alphabetic() {
            self.i += 1;
            return Some(c);
        }
        None
    }

    pub fn at_end(&mut self) -> bool {
        self.skip_sep();
        self.i >= self.s.len()
    }

    // Arc flags may run together ("11" is two flags), so read one digit.
    pub fn flag(&mut self) -> Option<bool> {
        self.skip_sep();
        let c = *self.s.as_bytes().get(self.i)?;
        if c == b'0' || c == b'1' {
            self.i += 1;
            return Some(c == b'1');
        }
        None
    }

    pub fn xy(&mut self, base: [f32; 2], rel: bool) -> Option<[f32; 2]> {
        let x = self.num()?;
        let y = self.num()?;
        if rel {
            Some([base[0] + x, base[1] + y])
        } else {
            Some([x, y])
        }
    }
}
