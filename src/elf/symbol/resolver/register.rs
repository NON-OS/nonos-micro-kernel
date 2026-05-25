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

use crate::memory::addr::VirtAddr;

use super::{resolved::ResolvedSymbol, state::SymbolResolver};

impl SymbolResolver {
    pub fn add_library(&mut self, library_id: usize) {
        if !self.library_order.contains(&library_id) {
            self.library_order.push(library_id);
        }
    }

    pub fn register_symbol(&mut self, symbol: ResolvedSymbol) {
        if symbol.is_weak() && !self.global_symbols.contains_key(&symbol.name) {
            self.weak_symbols.insert(symbol.name.clone(), symbol);
        } else if symbol.is_global() {
            self.weak_symbols.remove(&symbol.name);
            self.global_symbols.insert(symbol.name.clone(), symbol);
        }
    }

    pub fn resolve(&self, name: &str) -> Option<&ResolvedSymbol> {
        self.global_symbols.get(name).or_else(|| self.weak_symbols.get(name))
    }

    pub fn resolve_address(&self, name: &str) -> Option<VirtAddr> {
        self.resolve(name).map(|symbol| symbol.address)
    }
}
