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

//! Deterministic inbound TCP fault injector, compiled in ONLY under
//! the `tcp-chaos` feature. It drops a fixed set of inbound TCP
//! segments so the guest reliability machinery (RTO retransmit, peer
//! retransmit + dedup/reassembly) is forced to recover. The whole
//! module is feature-gated; with the feature off it is absent and the
//! normal ingress path is byte-for-byte unchanged.

use core::sync::atomic::{AtomicU32, Ordering};

static SEEN: AtomicU32 = AtomicU32::new(0);

const DROP_AT: &[u32] = &[3, 6];

pub fn should_drop_tcp() -> bool {
    let n = SEEN.fetch_add(1, Ordering::Relaxed) + 1;
    DROP_AT.contains(&n)
}
