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
use crate::server::tcp_rx::{rst, transitions};
use crate::state::Entry;
use crate::tcp::{State, TcpHeader, FLAG_ACK, FLAG_FIN, FLAG_RST};

pub fn step_entry(e: &mut Entry, hdr: &TcpHeader, payload: &[u8], now: u64, accepted: &mut Option<(u32, u32)>, arm: &mut Option<(u32, u64)>) -> RxAction {
    if hdr.has_flag(FLAG_RST) {
        return if rst::in_window(e, hdr.seq) { RxAction::Reap(e.handle) } else { RxAction::None };
    }
    if e.tcb.state == State::SynSent {
        return transitions::handshake::step(e, hdr);
    }
    if e.tcb.state == State::SynReceived && hdr.has_flag(FLAG_ACK) && hdr.ack == e.tcb.send.nxt {
        e.tcb.state = State::Established;
        *accepted = Some((e.parent, e.handle));
        if hdr.has_flag(FLAG_FIN) {
            e.tcb.recv.nxt = e.tcb.recv.nxt.wrapping_add(1);
            e.tcb.state = State::CloseWait;
            return RxAction::Reply(e.tcb, FLAG_ACK, Vec::new());
        }
        return RxAction::None;
    }
    if e.tcb.state.is_closing() {
        let (action, deadline) = transitions::closing::step(e, hdr, now);
        if let Some(d) = deadline {
            *arm = Some((e.handle, d));
        }
        return action;
    }
    if e.tcb.state.accepts_data() {
        return transitions::established::step(e, hdr, payload);
    }
    RxAction::None
}
