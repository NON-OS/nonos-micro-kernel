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

use super::types::{TRANSPARENT_MIN_PROOF_SIZE, ZK_PROOF_HEADER_SIZE, ZK_PROOF_MAGIC};

pub fn has_zk_proof(kernel_data: &[u8]) -> bool {
    if kernel_data.len() < ZK_PROOF_HEADER_SIZE + TRANSPARENT_MIN_PROOF_SIZE {
        return false;
    }

    let sig_offset =
        kernel_data.len().saturating_sub(64 + ZK_PROOF_HEADER_SIZE + TRANSPARENT_MIN_PROOF_SIZE);

    for offset in [0, sig_offset, sig_offset.saturating_sub(256), sig_offset.saturating_add(64)] {
        if offset + 4 <= kernel_data.len() {
            if &kernel_data[offset..offset + 4] == &ZK_PROOF_MAGIC {
                return true;
            }
        }
    }

    for i in (0..=kernel_data.len().saturating_sub(ZK_PROOF_HEADER_SIZE)).rev() {
        if kernel_data.len() - i < ZK_PROOF_HEADER_SIZE + TRANSPARENT_MIN_PROOF_SIZE {
            continue;
        }
        if &kernel_data[i..i + 4] == &ZK_PROOF_MAGIC {
            return true;
        }
        if kernel_data.len() - i > 8192 {
            break;
        }
    }

    false
}

pub fn find_zk_proof_offset(kernel_data: &[u8]) -> Option<usize> {
    if kernel_data.len() < ZK_PROOF_HEADER_SIZE + TRANSPARENT_MIN_PROOF_SIZE {
        return None;
    }
    if &kernel_data[0..4] == &ZK_PROOF_MAGIC {
        return Some(0);
    }
    for i in (0..=kernel_data.len().saturating_sub(ZK_PROOF_HEADER_SIZE)).rev() {
        if &kernel_data[i..i + 4] == &ZK_PROOF_MAGIC {
            return Some(i);
        }
        if kernel_data.len() - i > 8192 {
            break;
        }
    }
    None
}
