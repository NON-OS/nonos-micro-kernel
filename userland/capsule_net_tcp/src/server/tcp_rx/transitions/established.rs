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

use crate::server::tcp_rx::action::RxAction;
use crate::state::Entry;
use crate::tcp::{seq, State, TcpHeader, FLAG_ACK, FLAG_FIN};

pub fn step(e: &mut Entry, hdr: &TcpHeader, payload: &[u8]) -> RxAction {
    if !seq::acceptable(hdr.seq, payload.len() as u32, e.tcb.recv.nxt, e.tcb.recv.wnd) {
        e.tcb.recv.wnd = e.rwnd();
        return RxAction::Reply(e.tcb, FLAG_ACK, Vec::new());
    }
    if !payload.is_empty() && hdr.seq == e.tcb.recv.nxt {
        let _ = e.push_rx(payload);
        e.tcb.recv.nxt = e.tcb.recv.nxt.wrapping_add(payload.len() as u32);
    }
    if hdr.has_flag(FLAG_FIN) {
        e.tcb.recv.nxt = e.tcb.recv.nxt.wrapping_add(1);
        e.tcb.state = State::CloseWait;
    }
    e.tcb.recv.wnd = e.rwnd();
    RxAction::Reply(e.tcb, FLAG_ACK, Vec::new())
}
