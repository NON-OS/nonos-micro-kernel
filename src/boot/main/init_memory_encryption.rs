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

use crate::memory::encryption::{init_memory_encryption as detect_and_init, MemEncryption};
use crate::sys::serial;

pub(crate) fn init_memory_encryption() {
    match detect_and_init() {
        Ok(MemEncryption::None) => {
            serial::println(b"[NONOS] memory encryption: none");
        }
        Ok(MemEncryption::Pending(c_bit)) => {
            serial::print(b"[NONOS] memory encryption: SME C-bit=");
            serial::print(&[b'0' + (c_bit / 10), b'0' + (c_bit % 10)]);
            serial::println(b" (walk queued, live conversion deferred)");
        }
        Ok(MemEncryption::AmdSme) | Ok(MemEncryption::AmdSev) => {
            serial::println(b"[NONOS] memory encryption: AMD SME active");
        }
        Ok(MemEncryption::IntelTme) => {
            serial::println(b"[NONOS] memory encryption: Intel TME active");
        }
        Ok(MemEncryption::IntelMktme) => {
            serial::println(b"[NONOS] memory encryption: Intel MKTME active");
        }
        Err(_) => {
            serial::println(b"[NONOS] memory encryption: init failed; continuing plaintext");
        }
    }
}
