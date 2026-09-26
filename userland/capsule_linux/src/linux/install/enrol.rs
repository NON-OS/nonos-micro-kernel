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

//! Vouching for a program the machine just installed.

use alloc::vec::Vec;

use nonos_libc::{mk_local_sign, mk_local_sign_len};

use crate::linux::file::{key, store_write};

use crate::linux::attest_paths::beside;

/// The capabilities a guest is spawned with, which is none.
const GUEST_CAPS: u64 = 0;

/// Mint a trailer for `image` and write it beside `path`.
pub fn vouch(path: &[u8], image: &[u8]) -> bool {
    let needed = mk_local_sign_len(image, GUEST_CAPS);
    let Ok(len) = usize::try_from(needed) else {
        // Negative: no enrolled identity, or this capsule may not mint.
        return false;
    };
    let mut trailer: Vec<u8> = alloc::vec![0u8; len];
    let wrote = mk_local_sign(image, GUEST_CAPS, &mut trailer);
    let Ok(got) = usize::try_from(wrote) else {
        return false;
    };
    trailer.truncate(got);
    let at = beside(path, b".zk_trailer.bin");
    store_write(&key(&at), &trailer).is_ok()
}
