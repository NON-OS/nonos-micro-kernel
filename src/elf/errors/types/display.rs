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

impl core::fmt::Display for ElfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnsupportedRelocation(reloc_type) => {
                write!(f, "Unsupported relocation type: {}", reloc_type)
            }
            Self::Other(msg) => write!(f, "{}", msg),
            _ => write!(f, "{}", self.as_str()),
        }
    }
}

impl From<&'static str> for ElfError {
    fn from(value: &'static str) -> Self {
        ElfError::Other(value)
    }
}
