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

use super::read::read_name;
use super::state::SymbolLookup;
use alloc::string::String;

impl SymbolLookup {
    pub(super) fn name_or_empty(&self, offset: usize) -> String {
        if offset < self.strtab_size {
            unsafe { read_name(self.strtab, self.strtab_size, offset) }
        } else {
            String::new()
        }
    }

    pub fn symbol_count(&self) -> usize {
        self.sym_count
    }
}
