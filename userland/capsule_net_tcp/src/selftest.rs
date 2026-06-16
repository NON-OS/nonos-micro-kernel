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

#![cfg(feature = "tcp-selftest")]

use crate::tcp::seq;

pub fn run() -> u32 {
    let mut bits = 0u32;
    if seq_kat() {
        bits |= 1 << 0;
    }
    if accept_kat() {
        bits |= 1 << 1;
    }
    if rtt_kat() {
        bits |= 1 << 2;
    }
    if window_kat() {
        bits |= 1 << 3;
    }
    if retx_kat() {
        bits |= 1 << 4;
    }
    if reasm_kat() {
        bits |= 1 << 5;
    }
    bits
}

fn reasm_kat() -> bool {
    use crate::state::Reasm;
    let mut r = Reasm::new();
    r.insert(110, alloc::vec![1u8; 10]);
    if !r.drain_contiguous(100).is_empty() {
        return false;
    }
    r.insert(100, alloc::vec![0u8; 10]);
    r.drain_contiguous(100).len() == 20
}

fn retx_kat() -> bool {
    use crate::state::{RetxQueue, RetxSeg};
    let mut q = RetxQueue::new();
    q.push(RetxSeg { seq: 100, flags: 0, data: alloc::vec![0u8; 10], sent_ms: 0, xmits: 1 });
    q.push(RetxSeg { seq: 110, flags: 0, data: alloc::vec![0u8; 10], sent_ms: 0, xmits: 1 });
    q.ack(105) == 0 && q.ack(110) == 1 && q.ack(120) == 1 && q.is_empty()
}

fn window_kat() -> bool {
    use crate::tcp::window;
    window::should_update(100, 200, 110, 100, 199)
        && window::should_update(100, 200, 100, 100, 210)
        && !window::should_update(110, 200, 100, 100, 199)
        && window::usable(1000, 1000, 5000, 5000) == 5000
        && window::usable(1000, 4000, 5000, 5000) == 2000
        && window::usable(1000, 6000, 5000, 5000) == 0
}

fn rtt_kat() -> bool {
    use crate::tcp::rtt::Rtt;
    let mut r = Rtt::new();
    if r.rto_ms() != crate::tcp::RTO_INIT_MS {
        return false;
    }
    r.on_sample(100);
    if r.rto_ms() < crate::tcp::RTO_MIN_MS || r.rto_ms() > crate::tcp::RTO_MAX_MS {
        return false;
    }
    let a = r.rto_ms();
    r.backoff();
    r.rto_ms() == a.saturating_mul(2).min(crate::tcp::RTO_MAX_MS)
}

fn accept_kat() -> bool {
    seq::acceptable(100, 0, 100, 8)
        && seq::acceptable(100, 5, 100, 8)
        && !seq::acceptable(50, 0, 100, 8)
        && !seq::acceptable(200, 1, 100, 8)
        && seq::acceptable(100, 0, 100, 0)
        && !seq::acceptable(101, 0, 100, 0)
}

fn seq_kat() -> bool {
    seq::lt(1, 2)
        && !seq::lt(2, 1)
        && seq::lt(0xFFFF_FFFF, 0)
        && seq::leq(5, 5)
        && seq::gt(6, 5)
        && seq::geq(5, 5)
        && seq::geq(6, 5)
        && seq::between(5, 1, 10)
        && !seq::between(10, 1, 10)
}
