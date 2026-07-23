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

use alloc::vec;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_yield};

use super::wire;

const DRIVER_NAME: &[u8] = b"driver.hda0";
const LOOKUP_RETRIES: u32 = 2000;
const CALL_TIMEOUT_MS: u64 = 1000;

pub struct Sink {
    port: u32,
}

impl Sink {
    pub fn resolve() -> Option<Sink> {
        let mut tries = 0u32;
        while tries < LOOKUP_RETRIES {
            let mut port = 0u32;
            let mut pid = 0u32;
            let r = mk_service_lookup(DRIVER_NAME.as_ptr(), DRIVER_NAME.len(), &mut port, &mut pid);
            if r == 0 && port != 0 {
                return Some(Sink { port });
            }
            mk_yield();
            tries += 1;
        }
        None
    }

    pub fn write_pcm(&self, pcm: &[u8], request_id: u32) -> bool {
        let mut tx = vec![0u8; wire::HDR_LEN + pcm.len()];
        let n = wire::request(request_id, pcm, &mut tx);
        if n == 0 {
            return false;
        }
        let mut rx = vec![0u8; wire::HDR_LEN + wire::STATUS_LEN];
        let got = mk_ipc_call_timeout(
            self.port as u64,
            tx.as_ptr(),
            n,
            rx.as_mut_ptr(),
            rx.len(),
            CALL_TIMEOUT_MS,
        );
        if got <= 0 {
            return false;
        }
        wire::reply_ok(&rx[..got as usize])
    }
}
