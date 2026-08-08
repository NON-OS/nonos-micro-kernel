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

use super::state::ElfError;

impl ElfError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidMagic => "Invalid ELF magic number",
            Self::InvalidClass => "Invalid ELF class (not 64-bit)",
            Self::InvalidEndian => "Invalid ELF endianness (not little-endian)",
            Self::InvalidVersion => "Invalid ELF version",
            Self::InvalidMachine => "ELF machine type is not this architecture",
            Self::InvalidType => "Invalid ELF type (not EXEC or DYN)",
            Self::InvalidHeaderSize => "Invalid ELF header size",
            Self::InvalidProgramHeaderSize => "Invalid ELF program header entry size",
            Self::InvalidSectionHeaderSize => "Invalid ELF section header entry size",
            Self::FileTooSmall => "ELF file too small",
            Self::ProgramHeadersOutOfBounds => "Program headers out of bounds",
            Self::SectionHeadersOutOfBounds => "Section headers out of bounds",
            Self::SegmentDataOutOfBounds => "Segment data out of bounds",
            Self::MemoryAllocationFailed => "Memory allocation failed",
            Self::MemoryMappingFailed => "Memory mapping failed",
            Self::RelocationFailed => "Relocation processing failed",
            Self::UnsupportedRelocation(_) => "Unsupported relocation type",
            Self::InterpreterNotFound => "Interpreter not found",
            Self::InterpreterInvalidUtf8 => "Interpreter path not valid UTF-8",
            Self::TlsSectionError => "TLS section error",
            Self::DynamicSectionError => "Dynamic section error",
            Self::SymbolTableError => "Symbol table error",
            Self::SymbolNotFound => "Symbol not found",
            Self::StringTableError => "String table error",
            Self::StringTableOutOfBounds => "String table offset out of bounds",
            Self::UnknownFormat => "Unknown ELF format",
            Self::NotInitialized => "ELF loader not initialized",
            Self::AddressOverflow => "Address overflow",
            Self::AlignmentError => "Alignment requirements not met",
            Self::InvalidIndex => "Invalid index",
            Self::InvalidHash => "Invalid hash table",
            Self::InvalidAddress => "Invalid address",
            Self::InvalidState => "Invalid state",
            Self::LibraryNotFound => "Library not found",
            Self::LibraryAlreadyLoaded => "Library already loaded",
            Self::CircularDependency => "Circular dependency detected",
            Self::CacheFull => "Image cache full",
            Self::StackTooSmall => "Stack size too small",
            Self::WXViolation => "Segment requested both writable and executable permissions",
            Self::Other(msg) => msg,
        }
    }
}
