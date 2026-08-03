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

use super::super::chacha20::KEY_BYTES as STREAM_KEY_BYTES;
use super::types::{Lioness, KEY_BYTES, MAC_KEY_BYTES};
use super::wipe::wipe;

impl Lioness {
    pub fn new(key: &[u8; KEY_BYTES]) -> Self {
        let mut me = Self {
            k1: [0u8; STREAM_KEY_BYTES],
            k2: [0u8; MAC_KEY_BYTES],
            k3: [0u8; STREAM_KEY_BYTES],
            k4: [0u8; MAC_KEY_BYTES],
        };
        let s = STREAM_KEY_BYTES;
        let h = MAC_KEY_BYTES;
        me.k1.copy_from_slice(&key[..s]);
        me.k2.copy_from_slice(&key[s..s + h]);
        me.k3.copy_from_slice(&key[s + h..2 * s + h]);
        me.k4.copy_from_slice(&key[2 * s + h..]);
        me
    }
}

impl Drop for Lioness {
    fn drop(&mut self) {
        wipe(&mut self.k1);
        wipe(&mut self.k2);
        wipe(&mut self.k3);
        wipe(&mut self.k4);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
