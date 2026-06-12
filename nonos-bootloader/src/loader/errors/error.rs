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

use uefi::Status;

#[derive(Debug)]
pub enum LoaderError {
    InvalidMagic,
    InvalidClass,
    InvalidEndian,
    InvalidVersion,
    ElfParseError(&'static str),
    UnsupportedElf(&'static str),

    NoLoadableSegments,
    TooManySegments,
    SegmentOutOfBounds,
    SegmentOverlap,
    InvalidSegmentSize,
    InvalidSegmentAlignment,

    AllocationFailed { addr: u64, pages: usize, status: Status },
    AllocationTableFull,
    OutOfMemory,

    EntryNotInRange,
    AddressOutOfRange,
    IntegerOverflow,

    KernelTooLarge,
    WxViolation,
    MalformedElf(&'static str),
    SignatureInvalid,
    HashMismatch,

    CapsuleInvalid,
    CapsuleSignatureFailed,

    RelocationFailed(&'static str),
    UnsupportedRelocation(u32),
    SymbolNotFound,

    UefiError { desc: &'static str, status: Status },
    BootServicesUnavailable,

    InvalidDynamic,
    MissingDynamicInfo,

    FileNotFound,
    FileReadError,
    FileTooLarge,
}

pub type LoaderResult<T> = core::result::Result<T, LoaderError>;

impl From<uefi::Error> for LoaderError {
    fn from(e: uefi::Error) -> Self {
        LoaderError::UefiError { desc: "UEFI operation failed", status: e.status() }
    }
}
