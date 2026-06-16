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

use alloc::vec::Vec;

use crate::state::TABLE;
use crate::tcp::{State, MAX_RETX};

pub fn scan(now: u64) {
    let mut t = TABLE.lock();
    let mut abort: Vec<u32> = Vec::new();
    for e in t.entries_mut() {
        if e.tcb.state != State::Established || e.retx.is_empty() {
            continue;
        }
        let rto = e.rtt.rto_ms() as u64;
        let due = match e.retx.oldest_mut() {
            Some(seg) => now >= seg.sent_ms.saturating_add(rto),
            None => false,
        };
        if !due {
            continue;
        }
        let mut tcb = e.tcb;
        if let Some(seg) = e.retx.oldest_mut() {
            tcb.send.nxt = seg.seq;
            let _ = crate::server::tcp_tx::send(tcb, seg.flags, &seg.data);
            seg.xmits = seg.xmits.saturating_add(1);
            seg.sent_ms = now;
            if seg.xmits > MAX_RETX {
                abort.push(e.handle);
            }
        }
        e.rtt.backoff();
    }
    for h in abort {
        t.remove_by_handle(h);
        t.timers.cancel_all(h);
    }
}
