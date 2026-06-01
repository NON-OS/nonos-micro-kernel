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

mod core;
mod global;
mod image;

pub use self::core::{ElfLoader, ParsedSection};
pub(crate) use self::core::{parse_elf_header, parse_program_headers, validate_elf};
pub use self::global::{
    get_elf_loader, init_elf_loader, is_initialized, load_elf_entry_into, load_elf_executable,
    load_elf_executable_into,
};
pub use self::image::{DynamicInfo, ElfImage, LoadedSegment};
