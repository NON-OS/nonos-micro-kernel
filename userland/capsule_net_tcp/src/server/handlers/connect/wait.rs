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

use nonos_libc::mk_yield;

use crate::clock::now_ms;
use crate::server::{tcp_rx, tcp_tx};
use crate::state::TABLE;
use crate::tcp::{State, FLAG_SYN};

const FALLBACK_TRIES: usize = 8192;
const WAIT_MS: u64 = 8000;
const RESEND_MS: u64 = 1000;

pub fn established(owner: u32, handle: u32) -> bool {
    let start = now_ms();
    let mut fallback = 0;
    let mut last_resend = start;
    loop {
        tcp_rx::drain_one();
        let ready =
            TABLE.lock().owned_mut(owner, handle).map(|e| e.tcb.state == State::Established);
        if ready.map_or(false, |state| state) {
            return true;
        }
        if expired(start, &mut fallback) {
            return false;
        }
        let now = now_ms();
        if now.wrapping_sub(last_resend) >= RESEND_MS {
            last_resend = now;
            resend_syn(owner, handle);
        }
        mk_yield();
    }
}

fn resend_syn(owner: u32, handle: u32) {
    let tcb = {
        let mut table = TABLE.lock();
        match table.owned_mut(owner, handle) {
            Some(e) if e.tcb.state == State::SynSent => {
                let mut t = e.tcb;
                t.send.nxt = t.send.iss;
                t
            }
            _ => return,
        }
    };
    let _ = tcp_tx::send(tcb, FLAG_SYN, &[]);
}

fn expired(start: u64, fallback: &mut usize) -> bool {
    let now = now_ms();
    if start != 0 || now != 0 {
        return now.saturating_sub(start) >= WAIT_MS;
    }
    *fallback += 1;
    *fallback >= FALLBACK_TRIES
}
