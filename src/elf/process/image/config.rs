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

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ProcessConfig {
    pub name: String,
    pub args: Vec<String>,
    pub env: Vec<String>,
    pub stack_size: usize,
    pub uid: u32,
    pub gid: u32,
}

impl ProcessConfig {
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: Vec::new(),
            env: Vec::new(),
            stack_size: crate::elf::stack::DEFAULT_STACK_SIZE,
            uid: 0,
            gid: 0,
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn with_env(mut self, env: Vec<String>) -> Self {
        self.env = env;
        self
    }

    pub fn with_stack_size(mut self, size: usize) -> Self {
        self.stack_size = size;
        self
    }

    pub fn with_credentials(mut self, uid: u32, gid: u32) -> Self {
        self.uid = uid;
        self.gid = gid;
        self
    }
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self::new(String::new())
    }
}
