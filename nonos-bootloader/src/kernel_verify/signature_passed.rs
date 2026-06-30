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
use crate::log::logger::log_info;

pub fn signature_passed(result: &mut CryptoVerifyResult, key_id: &[u8], st: &mut SystemTable<Boot>) {
    result.signature_valid = true;
    log_info("kernel_verify", "Ed25519 signature VERIFIED against trusted key");
    print(st, cstr16!("  [CRYPTO] Ed25519 verify ....................... [PASS]\r\n"));
    mini_delay();
    print(st, cstr16!("  [CRYPTO] Signer key ID: "));
    print_hex_bytes(st, &key_id[0..8]);
    print(st, cstr16!("...\r\n"));
}
