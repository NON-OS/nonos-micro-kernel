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
    pub const fn is_validation_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidMagic
                | Self::InvalidClass
                | Self::InvalidEndian
                | Self::InvalidVersion
                | Self::InvalidMachine
                | Self::InvalidType
                | Self::InvalidHeaderSize
                | Self::InvalidProgramHeaderSize
                | Self::InvalidSectionHeaderSize
        )
    }
    pub const fn is_bounds_error(&self) -> bool {
        matches!(
            self,
            Self::FileTooSmall
                | Self::ProgramHeadersOutOfBounds
                | Self::SectionHeadersOutOfBounds
                | Self::SegmentDataOutOfBounds
                | Self::StringTableOutOfBounds
        )
    }
    pub const fn is_memory_error(&self) -> bool {
        matches!(
            self,
            Self::MemoryAllocationFailed | Self::MemoryMappingFailed | Self::AddressOverflow
        )
    }
    pub const fn is_dynamic_error(&self) -> bool {
        matches!(
            self,
            Self::RelocationFailed
                | Self::UnsupportedRelocation(_)
                | Self::DynamicSectionError
                | Self::SymbolTableError
                | Self::SymbolNotFound
                | Self::StringTableError
        )
    }
    pub const fn is_library_error(&self) -> bool {
        matches!(
            self,
            Self::LibraryNotFound
                | Self::LibraryAlreadyLoaded
                | Self::CircularDependency
                | Self::CacheFull
        )
    }
}
