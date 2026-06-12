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

use super::display::print;
use super::types::MIN_KERNEL_SIZE;
use crate::log::logger::log_error;

pub fn validate_kernel_size(kernel_data: &[u8], st: &mut SystemTable<Boot>) -> bool {
    if kernel_data.len() >= MIN_KERNEL_SIZE {
        return true;
    }
    log_error("crypto_real", "Kernel too small - no room for signature");
    print(st, cstr16!("  [CRYPTO] Kernel size check .................... [FAIL]\r\n"));
    print(st, cstr16!("  [CRYPTO] ERROR: Kernel too small for signature\r\n"));
    false
}
