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

//! Reading a Linux executable and putting it where a Linux program
//! expects to find itself.

mod auxv;
mod elf;
mod elf_phdr;
mod interp;
mod interp_ld;
mod load;
mod loaded;
mod phdr;
mod read;
mod segment;
mod stack;
mod stack_guard;
mod stack_strings;
mod stack_words;

pub use interp::{program, EXEC_BASE};
pub use stack::build;
