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

use crate::elf::embedded::EmbeddedLibraryRegistry;
use crate::memory::addr::VirtAddr;
use alloc::string::String;

use super::state::ProcessBuilder;
use super::super::image::ProcessConfig;

impl<'a> ProcessBuilder<'a> {
    pub fn with_config(mut self, config: ProcessConfig) -> Self {
        self.config = config;
        self
    }

    pub fn embedded_registry(mut self, registry: &'a EmbeddedLibraryRegistry) -> Self {
        self.embedded_registry = Some(registry);
        self
    }

    pub fn stack_top(mut self, addr: VirtAddr) -> Self {
        self.stack_top = addr;
        self
    }

    pub fn args(mut self, args: impl IntoIterator<Item = String>) -> Self {
        self.config.args = args.into_iter().collect();
        self
    }

    pub fn env(mut self, env: impl IntoIterator<Item = String>) -> Self {
        self.config.env = env.into_iter().collect();
        self
    }

    pub fn stack_size(mut self, size: usize) -> Self {
        self.config.stack_size = size;
        self
    }

    pub fn credentials(mut self, uid: u32, gid: u32) -> Self {
        self.config.uid = uid;
        self.config.gid = gid;
        self
    }
}
