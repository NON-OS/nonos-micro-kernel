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

use super::ops::connect;
use super::pick::pick;
use crate::state::TABLE;

/// Try the bootstrap gateway at `index`, wrapping around the list.
///
/// One per call so the caller keeps control between attempts. Wrapping means
/// a client that starts before the network is up keeps trying.
pub fn connect_candidate(tcp_port: u32, index: usize) -> bool {
    // Start somewhere random rather than at the head of the list. Walking it
    // in order means every machine enters the mixnet through the same gateway
    // on every boot, and an entry point that never changes is something an
    // observer can tie sessions together by.
    let Some(candidate) = pick(index, start_offset()) else {
        return false;
    };
    match connect(tcp_port, candidate) {
        Ok(gateway) => {
            let _ = TABLE.lock().set_gateway(gateway);
            super::trace::bound(gateway.ip);
            true
        }
        Err(_) => false,
    }
}

/// Where the walk begins, chosen once per boot from the same entropy the
/// packet keys come from.
fn start_offset() -> usize {
    use core::sync::atomic::{AtomicUsize, Ordering};
    static START: AtomicUsize = AtomicUsize::new(usize::MAX);
    let seen = START.load(Ordering::Relaxed);
    if seen != usize::MAX {
        return seen;
    }
    let mut byte = [0u8; 1];
    let picked = match crate::crypto::fill_random(&mut byte) {
        Ok(()) => byte[0] as usize,
        // No entropy is not a reason to refuse to connect, and the head of
        // the list is no worse than the old behaviour.
        Err(_) => 0,
    };
    START.store(picked, Ordering::Relaxed);
    picked
}
