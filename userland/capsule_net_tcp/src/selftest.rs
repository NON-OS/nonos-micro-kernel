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
    bits
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
