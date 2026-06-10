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

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsercopyError {
    NullPointer,
    InvalidAddress,
    AddressOverflow,
    MisalignedAddress,
    PageNotMapped,
    PageNotUser,
    PageNotWritable,
    PageTableCorrupt,
    PageFault,
    NoProcessContext,
    SizeTooLarge,
    InvalidUtf8,
}

impl fmt::Display for UsercopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NullPointer => write!(f, "null pointer"),
            Self::InvalidAddress => write!(f, "invalid user address"),
            Self::AddressOverflow => write!(f, "address overflow"),
            Self::MisalignedAddress => write!(f, "misaligned user address"),
            Self::PageNotMapped => write!(f, "page not mapped"),
            Self::PageNotUser => write!(f, "page not accessible from userspace"),
            Self::PageNotWritable => write!(f, "page not writable"),
            Self::PageTableCorrupt => write!(f, "page table outside directmap"),
            Self::PageFault => write!(f, "page fault during access"),
            Self::NoProcessContext => write!(f, "no process context"),
            Self::SizeTooLarge => write!(f, "copy size too large"),
            Self::InvalidUtf8 => write!(f, "invalid UTF-8 string"),
        }
    }
}

impl From<UsercopyError> for i32 {
    fn from(e: UsercopyError) -> i32 {
        match e {
            UsercopyError::NullPointer => -14,
            UsercopyError::InvalidAddress => -14,
            UsercopyError::AddressOverflow => -14,
            UsercopyError::MisalignedAddress => -14,
            UsercopyError::PageNotMapped => -14,
            UsercopyError::PageNotUser => -14,
            UsercopyError::PageNotWritable => -14,
            UsercopyError::PageTableCorrupt => -14,
            UsercopyError::PageFault => -14,
            UsercopyError::NoProcessContext => -3,
            UsercopyError::SizeTooLarge => -12,
            UsercopyError::InvalidUtf8 => -22,
        }
    }
}
