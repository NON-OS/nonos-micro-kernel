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

use super::state::SymbolResolver;

impl SymbolResolver {
    pub fn symbol_count(&self) -> usize {
        self.global_symbols.len() + self.weak_symbols.len()
    }
    pub fn global_count(&self) -> usize {
        self.global_symbols.len()
    }
    pub fn weak_count(&self) -> usize {
        self.weak_symbols.len()
    }
    pub fn library_count(&self) -> usize {
        self.library_order.len()
    }
}
