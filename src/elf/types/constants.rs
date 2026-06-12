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

pub mod class;
pub mod data;
pub mod dyn_tag;
mod elf_magic;
pub mod elf_osabi;
pub mod elf_type;
pub mod ident;
pub mod machine;
pub mod phdr_flags;
pub mod phdr_type;
pub mod reloc_type;
pub mod shdr_flags;
pub mod shdr_type;
pub mod sym_bind;
pub mod sym_type;

pub use class as elf_class;
pub use data as elf_data;
pub use elf_magic::ELF_MAGIC;
pub use machine as elf_machine;
pub use sym_bind as symbol_bind;
pub use sym_type as symbol_type;
