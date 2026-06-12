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

use super::types::{GROTH16_PROOF_SIZE, ZK_PROOF_HEADER_SIZE, ZK_PROOF_MAGIC};

/// Check if kernel data contains a ZK proof block
pub fn has_zk_proof(kernel_data: &[u8]) -> bool {
    // Minimum size check: signature + header + proof + some margin
    if kernel_data.len() < 64 + ZK_PROOF_HEADER_SIZE + GROTH16_PROOF_SIZE + 100 {
        return false;
    }

    // Calculate likely offset (before signature)
    let sig_offset =
        kernel_data.len().saturating_sub(64 + ZK_PROOF_HEADER_SIZE + GROTH16_PROOF_SIZE);

    // Check common locations first
    for offset in [sig_offset, sig_offset.saturating_sub(256), sig_offset.saturating_add(64)] {
        if offset + 4 <= kernel_data.len() {
            if &kernel_data[offset..offset + 4] == &ZK_PROOF_MAGIC {
                return true;
            }
        }
    }

    // Scan backwards from end (limited to 4KB)
    for i in (64..kernel_data.len().saturating_sub(ZK_PROOF_HEADER_SIZE)).rev() {
        if kernel_data.len() - i < ZK_PROOF_HEADER_SIZE + GROTH16_PROOF_SIZE {
            continue;
        }
        if &kernel_data[i..i + 4] == &ZK_PROOF_MAGIC {
            return true;
        }
        // Limit scan range
        if kernel_data.len() - i > 4096 {
            break;
        }
    }

    false
}

/// Find the offset of ZK proof magic in kernel data
pub fn find_zk_proof_offset(kernel_data: &[u8]) -> Option<usize> {
    // Scan backwards from end (limited to 8KB)
    for i in (64..kernel_data.len().saturating_sub(ZK_PROOF_HEADER_SIZE)).rev() {
        if &kernel_data[i..i + 4] == &ZK_PROOF_MAGIC {
            return Some(i);
        }
        if kernel_data.len() - i > 8192 {
            break;
        }
    }
    None
}
