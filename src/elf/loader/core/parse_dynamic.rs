// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

mod dynamic;
mod entry;
mod finalize;
mod interpreter;
mod offset;
mod state;
mod string;
mod tls;

pub(in crate::elf::loader::core) use dynamic::parse_dynamic_section;
pub(in crate::elf::loader::core) use interpreter::parse_interpreter;
pub(in crate::elf::loader::core) use string::read_string_from_data_limited;
pub(in crate::elf::loader::core) use tls::parse_tls_section;
