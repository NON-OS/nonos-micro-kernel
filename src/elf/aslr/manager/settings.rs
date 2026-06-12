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

use super::{entropy::gather_entropy, state::AslrManager};

impl AslrManager {
    pub fn new() -> Self {
        Self::with_settings(true, true, true)
    }
    pub fn with_settings(stack: bool, heap: bool, executable: bool) -> Self {
        Self {
            entropy_pool: gather_entropy(),
            stack_randomization: stack,
            heap_randomization: heap,
            executable_randomization: executable,
        }
    }
    pub fn disabled() -> Self {
        Self {
            entropy_pool: 0,
            stack_randomization: false,
            heap_randomization: false,
            executable_randomization: false,
        }
    }
    pub fn is_executable_randomization_enabled(&self) -> bool {
        self.executable_randomization
    }
    pub fn is_stack_randomization_enabled(&self) -> bool {
        self.stack_randomization
    }
    pub fn is_heap_randomization_enabled(&self) -> bool {
        self.heap_randomization
    }
    pub fn set_executable_randomization(&mut self, enabled: bool) {
        self.executable_randomization = enabled;
    }
    pub fn set_stack_randomization(&mut self, enabled: bool) {
        self.stack_randomization = enabled;
    }
    pub fn set_heap_randomization(&mut self, enabled: bool) {
        self.heap_randomization = enabled;
    }
}

impl Default for AslrManager {
    fn default() -> Self {
        Self::new()
    }
}
