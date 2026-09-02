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

//! The flag table a command declares, and the parse result it reads back.

use alloc::vec::Vec;

pub struct Spec<'s> {
    pub name: &'s [u8],
    pub bools: &'s [u8],
    pub valued: &'s [u8],
    pub words: &'s [&'s [u8]],
    pub numeric: u8,
}

impl<'s> Spec<'s> {
    pub fn new(name: &'s [u8], bools: &'s [u8]) -> Self {
        Spec { name, bools, valued: b"", words: &[], numeric: 0 }
    }

    pub fn valued(mut self, valued: &'s [u8]) -> Self {
        self.valued = valued;
        self
    }

    pub fn words(mut self, words: &'s [&'s [u8]]) -> Self {
        self.words = words;
        self
    }

    pub fn numeric(mut self, flag: u8) -> Self {
        self.numeric = flag;
        self
    }
}

#[derive(Debug, Default)]
pub struct Parsed<'a> {
    pub seen: Vec<u8>,
    pub vals: Vec<(u8, &'a [u8])>,
    pub wvals: Vec<(&'a [u8], &'a [u8])>,
    pub operands: Vec<&'a [u8]>,
}

impl<'a> Parsed<'a> {
    pub fn has(&self, flag: u8) -> bool {
        self.seen.contains(&flag)
    }

    pub fn value(&self, flag: u8) -> Option<&'a [u8]> {
        self.vals.iter().rev().find(|(k, _)| *k == flag).map(|(_, v)| *v)
    }

    pub fn word(&self, key: &[u8]) -> Option<&'a [u8]> {
        self.wvals.iter().rev().find(|(k, _)| *k == key).map(|(_, v)| *v)
    }
}
