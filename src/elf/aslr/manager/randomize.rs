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

use super::{constants::{EXEC_RANDOMIZATION_RANGE, HEAP_RANDOMIZATION_RANGE, STACK_RANDOMIZATION_RANGE}, entropy::gather_entropy, state::AslrManager};

impl AslrManager {
    pub fn randomize_base(&mut self, preferred_base: u64) -> u64 { if self.executable_randomization { (preferred_base + self.random_offset(EXEC_RANDOMIZATION_RANGE)) & !0xFFF } else { preferred_base } }
    pub fn randomize_stack(&mut self, base_stack: u64) -> u64 { if self.stack_randomization { (base_stack - self.random_offset(STACK_RANDOMIZATION_RANGE)) & !0xFFF } else { base_stack } }
    pub fn randomize_heap(&mut self, base_heap: u64) -> u64 { if self.heap_randomization { (base_heap + self.random_offset(HEAP_RANDOMIZATION_RANGE)) & !0xFFF } else { base_heap } }

    pub fn random_address(min: usize, max: usize, size: usize) -> usize {
        if min >= max || size == 0 || max - min <= size { return min; }
        let entropy = match usize::try_from(gather_entropy() >> 16) { Ok(value) => value, Err(_) => 0 };
        let offset = entropy % (max - min - size);
        (min + offset) & !0xFFF
    }
}
