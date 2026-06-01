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
use crate::elf::loader::ElfLoader;
use crate::memory::addr::VirtAddr;
use alloc::string::String;

use super::super::image::ProcessConfig;

pub const DEFAULT_USER_STACK_TOP: u64 = 0x7FFF_FFFF_F000;

pub struct ProcessBuilder<'a> {
    pub(super) loader: &'a mut ElfLoader,
    pub(super) embedded_registry: Option<&'a EmbeddedLibraryRegistry>,
    pub(super) config: ProcessConfig,
    pub(super) stack_top: VirtAddr,
}

impl<'a> ProcessBuilder<'a> {
    pub fn new(loader: &'a mut ElfLoader, name: String) -> Self {
        Self {
            loader,
            embedded_registry: None,
            config: ProcessConfig::new(name),
            stack_top: VirtAddr::new(DEFAULT_USER_STACK_TOP),
        }
    }
}
