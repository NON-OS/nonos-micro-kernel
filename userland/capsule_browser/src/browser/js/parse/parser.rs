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

use crate::browser::js::token::Tok;

pub struct Parser {
    pub toks: Vec<Tok>,
    pub pos: usize,
    pub depth: u32,
}

impl Parser {
    pub fn new(toks: Vec<Tok>) -> Self {
        Parser { toks, pos: 0, depth: 0 }
    }
    pub fn peek(&self) -> &Tok {
        &self.toks[self.pos.min(self.toks.len() - 1)]
    }
    pub fn advance(&mut self) -> Tok {
        let t = self.peek().clone();
        if self.pos < self.toks.len() - 1 {
            self.pos += 1;
        }
        t
    }
    pub fn is_punct(&self, s: &str) -> bool {
        matches!(self.peek(), Tok::Punct(p) if p == s)
    }
    pub fn eat_punct(&mut self, s: &str) -> bool {
        if self.is_punct(s) {
            self.advance();
            true
        } else {
            false
        }
    }
    pub fn is_kw(&self, k: &str) -> bool {
        matches!(self.peek(), Tok::Ident(i) if i == k)
    }
    pub fn eat_kw(&mut self, k: &str) -> bool {
        if self.is_kw(k) {
            self.advance();
            true
        } else {
            false
        }
    }
}
