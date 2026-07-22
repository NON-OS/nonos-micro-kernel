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

// RFC 2104 HMAC over SHA-512, streaming and allocation-free: the message is
// absorbed straight into the inner hash, so no key- or message-sized buffer
// is ever built. Pads are wiped when the MAC is finalized.

use crate::sha512::{sha512, Sha512};
use crate::wipe::wipe;

pub struct HmacSha512 {
    inner: Sha512,
    opad: [u8; 128],
}

impl HmacSha512 {
    pub fn new(key: &[u8]) -> Self {
        let mut k = [0u8; 128];
        if key.len() > 128 {
            let digest = sha512(key);
            k[..64].copy_from_slice(&digest);
        } else {
            k[..key.len()].copy_from_slice(key);
        }

        let mut ipad = [0u8; 128];
        let mut opad = [0u8; 128];
        for i in 0..128 {
            ipad[i] = k[i] ^ 0x36;
            opad[i] = k[i] ^ 0x5c;
        }
        wipe(&mut k);

        let mut inner = Sha512::new();
        inner.update(&ipad);
        wipe(&mut ipad);

        Self { inner, opad }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub fn finalize(mut self) -> [u8; 64] {
        let inner = core::mem::take(&mut self.inner);
        let inner_hash = inner.finalize();
        let mut outer = Sha512::new();
        outer.update(&self.opad);
        outer.update(&inner_hash);
        outer.finalize()
        // Drop wipes opad; the replaced inner hasher wipes itself.
    }
}

impl Drop for HmacSha512 {
    fn drop(&mut self) {
        // The pads are key-derived, so a MAC abandoned before finalize must
        // not leave them behind.
        wipe(&mut self.opad);
    }
}

pub fn hmac_sha512(key: &[u8], message: &[u8]) -> [u8; 64] {
    let mut mac = HmacSha512::new(key);
    mac.update(message);
    mac.finalize()
}
