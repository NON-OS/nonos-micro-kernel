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

mod bounds;
mod header;
mod program_entry;
mod program_headers;
mod validate;

pub(crate) use bounds::program_header_bounds;
pub(crate) use header::parse_elf_header;
pub(crate) use program_entry::parse_program_header_at;
pub(crate) use program_headers::parse_program_headers;
pub(crate) use validate::validate_elf;
