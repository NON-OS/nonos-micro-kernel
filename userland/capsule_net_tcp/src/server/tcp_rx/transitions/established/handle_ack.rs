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

use crate::state::Entry;
use crate::tcp::{seq, TcpHeader, FLAG_ACK};

pub fn handle_ack(e: &mut Entry, hdr: &TcpHeader) -> bool {
    if !(hdr.has_flag(FLAG_ACK) && seq::gt(hdr.ack, e.tcb.send.una) && seq::leq(hdr.ack, e.tcb.send.nxt)) {
        return false;
    }
    if let Some(oldest) = e.retx.oldest_mut() {
        if oldest.xmits == 1 {
            let r = crate::clock::now_ms().saturating_sub(oldest.sent_ms).min(crate::tcp::RTO_MAX_MS as u64) as u32;
            e.rtt.on_sample(r);
        }
    }
    e.tcb.send.una = hdr.ack;
    e.retx.ack(hdr.ack);
    e.cc.on_new_ack();
    if crate::tcp::window::should_update(e.tcb.send.wl1, e.tcb.send.wl2, hdr.seq, e.tcb.send.una, hdr.ack) {
        e.tcb.send.wnd = hdr.window;
        e.tcb.send.wl1 = hdr.seq;
        e.tcb.send.wl2 = hdr.ack;
    }
    crate::server::sender::drain_send(e);
    true
}
