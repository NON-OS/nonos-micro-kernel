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


//! Bytes in and out of a guest, a megabyte at a time.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::peer::{mk_peer_read, mk_peer_write};

use super::handle::Guest;
use super::mem::MAX_SPAN;

impl Guest {
    /// Bytes written, or the first failure.
    pub fn write(&self, addr: u64, bytes: &[u8]) -> i64 {
        let mut done = 0usize;
        while done < bytes.len() {
            let take = (bytes.len() - done).min(MAX_SPAN as usize);
            let rc = mk_peer_write(self.pid, addr + done as u64, &bytes[done..done + take]);
            if rc < 0 {
                return rc;
            }
            done += take;
        }
        bytes.len() as i64
    }

    pub fn read(&self, addr: u64, len: usize) -> Option<Vec<u8>> {
        let mut out = vec![0u8; len];
        let mut done = 0usize;
        while done < len {
            let take = (len - done).min(MAX_SPAN as usize);
            if mk_peer_read(self.pid, addr + done as u64, &mut out[done..done + take]) < 0 {
                return None;
            }
            done += take;
        }
        Some(out)
    }
}
