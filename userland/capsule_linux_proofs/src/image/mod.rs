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


//! The loader's module shape, so the `super::` paths inside these files
//! resolve exactly as they do in the capsule.

#[path = "../../../capsule_linux/src/linux/image/read.rs"]
pub mod read;

#[path = "../../../capsule_linux/src/linux/image/phdr.rs"]
pub mod phdr;

#[path = "../../../capsule_linux/src/linux/image/stack_words.rs"]
pub mod stack_words;

#[path = "../../../capsule_linux/src/linux/image/elf.rs"]
pub mod elf;

#[path = "../../../capsule_linux/src/linux/image/elf_phdr.rs"]
pub mod elf_phdr;
