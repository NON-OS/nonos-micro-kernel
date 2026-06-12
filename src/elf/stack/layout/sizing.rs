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

use super::config::StackConfig;
use super::constants::{MIN_STACK_SIZE, POINTER_SIZE, STACK_ALIGNMENT};
use crate::elf::auxv::AuxEntry;
use alloc::string::String;

impl StackConfig {
    pub fn with_stack_size(mut self, size: usize) -> Self {
        self.stack_size = size.max(MIN_STACK_SIZE);
        self
    }

    pub fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }

    pub fn add_env(&mut self, key: &str, value: &str) {
        let mut entry = String::with_capacity(key.len() + value.len() + 1);
        entry.push_str(key);
        entry.push('=');
        entry.push_str(value);
        self.env.push(entry);
    }

    pub fn argc(&self) -> usize {
        self.args.len()
    }

    pub fn strings_size(&self) -> usize {
        self.args.iter().map(|s| s.len() + 1).sum::<usize>()
            + self.env.iter().map(|s| s.len() + 1).sum::<usize>()
    }

    pub fn pointers_size(&self) -> usize {
        POINTER_SIZE
            + (self.args.len() + 1) * POINTER_SIZE
            + (self.env.len() + 1) * POINTER_SIZE
            + self.auxv.len() * AuxEntry::SIZE
    }

    pub fn total_setup_size(&self) -> usize {
        self.strings_size() + self.pointers_size() + STACK_ALIGNMENT
    }
}
