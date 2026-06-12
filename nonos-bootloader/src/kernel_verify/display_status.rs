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
use crate::boot::util::print_u64;

pub fn print_kernel_size(st: &mut SystemTable<Boot>, size: usize) {
    print(st, cstr16!("  [CRYPTO] Kernel bytes: "));
    print_u64(st, size as u64);
    print(st, cstr16!("\r\n"));
}

pub fn print_verification_success(st: &mut SystemTable<Boot>) {
    print(st, cstr16!("  [CRYPTO] Kernel signature state ............... [PASS]\r\n"));
}

pub fn print_verification_failure(st: &mut SystemTable<Boot>) {
    print(st, cstr16!("  [CRYPTO] Kernel signature state ............... [FAIL]\r\n"));
}
