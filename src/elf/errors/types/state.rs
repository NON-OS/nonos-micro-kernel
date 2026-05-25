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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfError {
    InvalidMagic, InvalidClass, InvalidEndian, InvalidVersion, InvalidMachine, InvalidType, InvalidHeaderSize,
    InvalidProgramHeaderSize, InvalidSectionHeaderSize, FileTooSmall,
    ProgramHeadersOutOfBounds, SectionHeadersOutOfBounds, SegmentDataOutOfBounds, MemoryAllocationFailed, MemoryMappingFailed,
    RelocationFailed, UnsupportedRelocation(u32), InterpreterNotFound, InterpreterInvalidUtf8, TlsSectionError,
    DynamicSectionError, SymbolTableError, SymbolNotFound, StringTableError, StringTableOutOfBounds, UnknownFormat,
    NotInitialized, AddressOverflow, AlignmentError, InvalidIndex, InvalidHash, InvalidAddress, InvalidState,
    LibraryNotFound, LibraryAlreadyLoaded, CircularDependency, CacheFull, StackTooSmall, WXViolation, Other(&'static str),
}
