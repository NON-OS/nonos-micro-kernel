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

use super::compile::compile;
use super::find::{search, Match};
use super::inst::Inst;
use super::parse::parse;

// A compiled regular expression plus the flags that shape matching.
pub struct Regex {
    prog: Vec<Inst>,
    ngroups: usize,
    pub global: bool,
    pub ci: bool,
}

impl Regex {
    pub fn compile(pattern: &str, flags: &str) -> Regex {
        let (re, ngroups) = parse(pattern);
        Regex { prog: compile(&re), ngroups, global: flags.contains('g'), ci: flags.contains('i') }
    }
    pub fn find(&self, text: &[char], start: usize) -> Option<Match> {
        search(&self.prog, self.ngroups, text, start, self.ci)
    }
}
