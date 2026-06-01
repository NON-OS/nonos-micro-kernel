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
use crate::elf::errors::ElfResult;
use crate::elf::loader::ElfLoader;
use alloc::string::String;

use super::state::ProcessBuilder;
use super::super::image::ProcessImage;

pub fn create_process(loader: &mut ElfLoader, name: String, elf_data: &[u8]) -> ElfResult<ProcessImage> {
    ProcessBuilder::new(loader, name).build(elf_data)
}

pub fn create_process_with_registry(
    loader: &mut ElfLoader,
    registry: &EmbeddedLibraryRegistry,
    name: String,
    elf_data: &[u8],
) -> ElfResult<ProcessImage> {
    ProcessBuilder::new(loader, name).embedded_registry(registry).build(elf_data)
}

pub fn create_process_with_args(
    loader: &mut ElfLoader,
    name: String,
    elf_data: &[u8],
    args: impl IntoIterator<Item = String>,
    env: impl IntoIterator<Item = String>,
) -> ElfResult<ProcessImage> {
    ProcessBuilder::new(loader, name).args(args).env(env).build(elf_data)
}

pub fn create_process_with_args_and_registry(
    loader: &mut ElfLoader,
    registry: &EmbeddedLibraryRegistry,
    name: String,
    elf_data: &[u8],
    args: impl IntoIterator<Item = String>,
    env: impl IntoIterator<Item = String>,
) -> ElfResult<ProcessImage> {
    ProcessBuilder::new(loader, name)
        .embedded_registry(registry)
        .args(args)
        .env(env)
        .build(elf_data)
}
