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
use alloc::vec::Vec;

// A parsed regular expression node.
#[derive(Clone)]
pub enum Re {
    Char(char),
    Any,
    Class(Vec<ClassItem>, bool),
    Start,
    End,
    WordB(bool),
    Group(Box<Re>, Option<usize>),
    Concat(Vec<Re>),
    Alt(Vec<Re>),
    Repeat(Box<Re>, usize, Option<usize>, bool),
}

// One entry inside a `[...]` class; the bool on the shorthands marks negation.
#[derive(Clone)]
pub enum ClassItem {
    Ch(char),
    Range(char, char),
    Digit(bool),
    Word(bool),
    Space(bool),
}
