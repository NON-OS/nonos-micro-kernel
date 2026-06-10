// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::error::LoaderError;

impl fmt::Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoaderError::InvalidMagic => write!(f, "invalid ELF magic number"),
            LoaderError::InvalidClass => write!(f, "invalid ELF class (not 64-bit)"),
            LoaderError::InvalidEndian => write!(f, "invalid ELF endianness"),
            LoaderError::InvalidVersion => write!(f, "invalid ELF version"),
            LoaderError::ElfParseError(s) => write!(f, "ELF parse failed: {}", s),
            LoaderError::UnsupportedElf(s) => write!(f, "unsupported ELF: {}", s),

            LoaderError::NoLoadableSegments => write!(f, "no PT_LOAD segments found"),
            LoaderError::TooManySegments => write!(f, "too many PT_LOAD segments"),
            LoaderError::SegmentOutOfBounds => write!(f, "segment extends beyond file"),
            LoaderError::SegmentOverlap => write!(f, "overlapping segments detected"),
            LoaderError::InvalidSegmentSize => {
                write!(f, "invalid segment size (p_memsz < p_filesz)")
            }
            LoaderError::InvalidSegmentAlignment => write!(f, "invalid segment alignment"),

            LoaderError::AllocationFailed { addr, pages, status } => {
                write!(f, "allocation failed at 0x{:x} ({} pages): {:?}", addr, pages, status)
            }
            LoaderError::AllocationTableFull => write!(f, "allocation table full"),
            LoaderError::OutOfMemory => write!(f, "out of memory"),

            LoaderError::EntryNotInRange => write!(f, "entry point not in loaded range"),
            LoaderError::AddressOutOfRange => write!(f, "SECURITY: address outside allowed range"),
            LoaderError::IntegerOverflow => write!(f, "SECURITY: integer overflow"),

            LoaderError::KernelTooLarge => write!(f, "SECURITY: kernel exceeds maximum size"),
            LoaderError::WxViolation => write!(f, "SECURITY: W^X violation"),
            LoaderError::MalformedElf(s) => write!(f, "SECURITY: malformed ELF - {}", s),
            LoaderError::SignatureInvalid => write!(f, "SECURITY: signature verification failed"),
            LoaderError::HashMismatch => write!(f, "SECURITY: hash mismatch"),

            LoaderError::CapsuleInvalid => write!(f, "capsule validation failed"),
            LoaderError::CapsuleSignatureFailed => write!(f, "capsule signature invalid"),

            LoaderError::RelocationFailed(s) => write!(f, "relocation failed: {}", s),
            LoaderError::UnsupportedRelocation(t) => {
                write!(f, "unsupported relocation type: {}", t)
            }
            LoaderError::SymbolNotFound => write!(f, "symbol not found"),

            LoaderError::UefiError { desc, status } => write!(f, "{}: {:?}", desc, status),
            LoaderError::BootServicesUnavailable => write!(f, "boot services unavailable"),

            LoaderError::InvalidDynamic => write!(f, "invalid dynamic section"),
            LoaderError::MissingDynamicInfo => write!(f, "missing required dynamic info"),

            LoaderError::FileNotFound => write!(f, "file not found"),
            LoaderError::FileReadError => write!(f, "file read error"),
            LoaderError::FileTooLarge => write!(f, "file too large"),
        }
    }
}
