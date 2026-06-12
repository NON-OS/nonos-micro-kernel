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

use uefi::cstr16;
use uefi::prelude::*;

use super::delay::mini_delay;
use super::display::{print, print_hex_bytes};
use super::types::CryptoVerifyResult;
use crate::log::logger::log_debug;

pub fn compute_and_display_hash(
    kernel_code: &[u8],
    result: &mut CryptoVerifyResult,
    st: &mut SystemTable<Boot>,
) {
    print(st, cstr16!("  [CRYPTO] Computing BLAKE3 hash...\r\n"));
    mini_delay();
    let hash = blake3::hash(kernel_code);
    let hash_bytes = hash.as_bytes();
    result.kernel_hash_full.copy_from_slice(hash_bytes);
    result.kernel_hash_preview.copy_from_slice(&hash_bytes[..8]);
    log_debug("kernel_verify", "BLAKE3 hash computed");
    print(st, cstr16!("  [CRYPTO] BLAKE3: "));
    print_hex_bytes(st, &hash_bytes[..8]);
    print(st, cstr16!("...\r\n"));
    mini_delay();
}
