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

use super::capsule::verify_signature_full;
use super::metadata::CapsuleMetadata;
use crate::log::logger::{log_info, log_warn};

pub fn verify_signature(blob: &[u8], meta: &CapsuleMetadata) -> bool {
    match verify_signature_full(blob, meta) {
        Ok(_) => {
            log_info("crypto", "signature verified");
            true
        }
        Err(_) => {
            log_warn("crypto", "signature verification failed");
            false
        }
    }
}
