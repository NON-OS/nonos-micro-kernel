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

mod checks;
mod entry;
mod parse;
mod queries;

pub use checks::{validate_elf, validate_elf_detailed, validate_elf_x86_64};
pub use entry::entry_from_bytes;
pub use queries::{
    get_elf_machine, get_elf_type, get_phnum, get_phoff, get_shnum, get_shoff, is_pie,
};
