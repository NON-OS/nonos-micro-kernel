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

//! The module matrix and its cell accessors.

use alloc::vec::Vec;

pub(crate) struct Matrix {
    pub n: usize,
    // Row-major; true = dark.
    pub modules: Vec<bool>,
    // true where a function pattern or reserved area sits; data and masking
    // never touch these.
    pub function: Vec<bool>,
}

impl Matrix {
    pub(super) fn set_fn(&mut self, x: usize, y: usize, dark: bool) {
        self.modules[y * self.n + x] = dark;
        self.function[y * self.n + x] = true;
    }

    pub(crate) fn get(&self, x: usize, y: usize) -> bool {
        self.modules[y * self.n + x]
    }

    // Write a format/version-info module (its cell is already reserved).
    pub(crate) fn set_format(&mut self, x: usize, y: usize, dark: bool) {
        self.modules[y * self.n + x] = dark;
        self.function[y * self.n + x] = true;
    }
}
