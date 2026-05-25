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

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::resolved::ResolvedSymbol;

pub struct SymbolResolver {
    pub(super) global_symbols: BTreeMap<String, ResolvedSymbol>,
    pub(super) weak_symbols: BTreeMap<String, ResolvedSymbol>,
    pub(super) library_order: Vec<usize>,
}

impl SymbolResolver {
    pub fn new() -> Self {
        Self { global_symbols: BTreeMap::new(), weak_symbols: BTreeMap::new(), library_order: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.global_symbols.clear();
        self.weak_symbols.clear();
        self.library_order.clear();
    }
}

impl Default for SymbolResolver {
    fn default() -> Self { Self::new() }
}
