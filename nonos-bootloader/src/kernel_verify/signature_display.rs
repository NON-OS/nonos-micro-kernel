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

pub fn display_signature_components(signature: &[u8], st: &mut SystemTable<Boot>) {
    print(st, cstr16!("  [CRYPTO] Sig R: "));
    print_hex_bytes(st, &signature[0..8]);
    print(st, cstr16!("...\r\n"));
    print(st, cstr16!("  [CRYPTO] Sig S: "));
    print_hex_bytes(st, &signature[32..40]);
    print(st, cstr16!("...\r\n"));
    mini_delay();
}
