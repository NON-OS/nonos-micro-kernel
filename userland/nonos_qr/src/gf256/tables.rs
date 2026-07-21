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

//! GF(2^8) log/antilog tables over the QR primitive polynomial 0x11D.

pub(super) struct Tables {
    pub exp: [u8; 256],
    pub log: [u8; 256],
}

// A pure function of the field, so a single lazily-filled static is safe: every
// caller computes the identical bytes.
pub(super) fn tables() -> &'static Tables {
    use core::sync::atomic::{AtomicBool, Ordering};
    static mut T: Tables = Tables { exp: [0; 256], log: [0; 256] };
    static INIT: AtomicBool = AtomicBool::new(false);
    // SAFETY: filled once; the Release store publishes a fully initialized
    // table that later readers observe through the Acquire load.
    unsafe {
        if !INIT.load(Ordering::Acquire) {
            let mut exp = [0u8; 256];
            let mut log = [0u8; 256];
            let mut x: u16 = 1;
            for i in 0..255 {
                exp[i] = x as u8;
                log[x as usize] = i as u8;
                x <<= 1;
                if x & 0x100 != 0 {
                    x ^= 0x11D;
                }
            }
            exp[255] = exp[0];
            T = Tables { exp, log };
            INIT.store(true, Ordering::Release);
        }
        &*core::ptr::addr_of!(T)
    }
}
