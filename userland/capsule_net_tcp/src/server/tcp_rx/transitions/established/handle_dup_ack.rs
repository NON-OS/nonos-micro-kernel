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
use crate::tcp::{TcpHeader, FLAG_ACK};

pub fn handle_dup_ack(e: &mut Entry, hdr: &TcpHeader, payload: &[u8]) {
    if !(hdr.has_flag(FLAG_ACK) && hdr.ack == e.tcb.send.una && hdr.window == e.tcb.send.wnd && payload.is_empty()) {
        return;
    }
    if e.retx.is_empty() || !e.cc.on_dup_ack() {
        return;
    }
    let mut t = e.tcb;
    if let Some(seg) = e.retx.oldest_mut() {
        t.send.nxt = seg.seq;
        let _ = crate::server::tcp_tx::send(t, seg.flags, &seg.data);
        seg.sent_ms = crate::clock::now_ms();
    }
}
