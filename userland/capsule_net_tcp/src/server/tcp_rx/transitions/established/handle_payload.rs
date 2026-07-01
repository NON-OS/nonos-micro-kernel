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

use crate::state::{Entry, RX_DEPTH};
use crate::tcp::{seq, TcpHeader};

pub fn handle_payload(e: &mut Entry, hdr: &TcpHeader, payload: &[u8]) {
    if payload.is_empty() {
        return;
    }
    if hdr.seq == e.tcb.recv.nxt && e.rx.len() < RX_DEPTH && e.push_rx(payload) {
        e.tcb.recv.nxt = e.tcb.recv.nxt.wrapping_add(payload.len() as u32);
        while e.rx.len() < RX_DEPTH {
            let more = e.reasm.drain_contiguous(e.tcb.recv.nxt);
            if more.is_empty() || !e.push_rx(&more) {
                break;
            }
            e.tcb.recv.nxt = e.tcb.recv.nxt.wrapping_add(more.len() as u32);
        }
    } else if seq::gt(hdr.seq, e.tcb.recv.nxt) {
        e.reasm.insert(hdr.seq, payload.to_vec());
    }
}
