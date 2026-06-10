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

use super::checksum::{verify_checksum, ChecksumType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityResult {
    Valid,
    InvalidMagic,
    InvalidChecksum,
    InvalidSize,
    Corrupted,
}

pub fn validate_firmware_integrity(data: &[u8], expected_checksum: &[u8]) -> IntegrityResult {
    if data.len() < 16 {
        return IntegrityResult::InvalidSize;
    }
    if !verify_header_magic(data) {
        return IntegrityResult::InvalidMagic;
    }
    if !verify_checksum(data, expected_checksum, ChecksumType::Sha256) {
        return IntegrityResult::InvalidChecksum;
    }
    if detect_corruption(data) {
        return IntegrityResult::Corrupted;
    }
    IntegrityResult::Valid
}

pub fn verify_header_magic(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }
    let magic = &data[0..4];
    matches!(magic, b"\x7fELF" | b"UEFI" | b"ACPI" | b"INTL" | b"RTK\x00")
}

fn detect_corruption(data: &[u8]) -> bool {
    let zero_blocks = data.chunks(1024).filter(|chunk| chunk.iter().all(|&b| b == 0)).count();
    let ff_blocks = data.chunks(1024).filter(|chunk| chunk.iter().all(|&b| b == 0xFF)).count();
    let total_blocks = (data.len() + 1023) / 1024;
    (zero_blocks + ff_blocks) > total_blocks / 3
}
