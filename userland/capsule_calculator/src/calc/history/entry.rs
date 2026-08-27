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

use crate::calc::fixed::Fixed;

pub const EXPR_MAX: usize = 40;

#[derive(Clone, Copy)]
pub struct Entry {
    expr: [u8; EXPR_MAX],
    len: u8,
    pub value: Fixed,
}

impl Entry {
    pub const fn empty() -> Self {
        Entry { expr: [0u8; EXPR_MAX], len: 0, value: 0 }
    }
    pub fn new(expr: &[u8], value: Fixed) -> Self {
        let mut built = Entry::empty();
        let n = expr.len().min(EXPR_MAX);
        built.expr[..n].copy_from_slice(&expr[..n]);
        built.len = n as u8;
        built.value = value;
        built
    }
    pub fn text(&self) -> &str {
        core::str::from_utf8(&self.expr[..self.len as usize]).unwrap_or("")
    }
}
