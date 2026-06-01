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

use super::constants::DEFAULT_STACK_SIZE;
use crate::elf::auxv::AuxEntry;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct StackConfig {
    pub args: Vec<String>,
    pub env: Vec<String>,
    pub auxv: Vec<AuxEntry>,
    pub stack_size: usize,
}

impl StackConfig {
    pub fn new() -> Self {
        Self { args: Vec::new(), env: Vec::new(), auxv: Vec::new(), stack_size: DEFAULT_STACK_SIZE }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn with_env(mut self, env: Vec<String>) -> Self {
        self.env = env;
        self
    }

    pub fn with_auxv(mut self, auxv: Vec<AuxEntry>) -> Self {
        self.auxv = auxv;
        self
    }
}

impl Default for StackConfig {
    fn default() -> Self {
        Self::new()
    }
}
