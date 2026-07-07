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

// The real ELF error and type definitions (self-contained), plus the header and
// program-header-bounds parsing the capsule loader runs on untrusted ELF bytes.
#[path = "../../../../src/elf/errors/mod.rs"]
pub mod errors;

// The manual Default impls are the real code's own choice.
#[allow(clippy::derivable_impls)]
#[path = "../../../../src/elf/types/mod.rs"]
pub mod types;

pub mod loader;

use errors::ElfError;
use types::ElfHeader;

// Public surface for the proofs: parse the fixed ELF header, and compute the
// program-header table bounds, from untrusted bytes.
pub fn parse_header(bytes: &[u8]) -> Result<ElfHeader, ElfError> {
    loader::core::parse_header::parse_elf_header(bytes)
}

pub fn program_header_bounds(
    bytes: &[u8],
    header: &ElfHeader,
) -> Result<(usize, usize, usize), ElfError> {
    loader::core::parse_header::program_header_bounds(bytes, header)
}
