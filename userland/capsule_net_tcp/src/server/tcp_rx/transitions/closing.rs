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

pub fn step(e: &mut Entry, hdr: &TcpHeader, now_ms: u64) -> (RxAction, Option<u64>) {
    let seg_len = if hdr.has_flag(FLAG_FIN) { 1 } else { 0 };
    if !seq::acceptable(hdr.seq, seg_len, e.tcb.recv.nxt, e.tcb.recv.wnd) {
        return (RxAction::Reply(e.tcb, FLAG_ACK, Vec::new()), None);
    }
    let acks_fin = hdr.has_flag(FLAG_ACK) && hdr.ack == e.tcb.send.nxt;
    let has_fin = hdr.has_flag(FLAG_FIN);
    if acks_fin {
        e.tcb.send.una = hdr.ack;
    }
    match e.tcb.state {
        State::FinWait1 if acks_fin && has_fin => to_timewait(e, hdr, now_ms),
        State::FinWait1 if acks_fin => {
            e.tcb.state = State::FinWait2;
            (RxAction::None, None)
        }
        State::FinWait1 if has_fin => {
            e.tcb.recv.nxt = hdr.seq.wrapping_add(1);
            e.tcb.state = State::Closing;
            (RxAction::Reply(e.tcb, FLAG_ACK, Vec::new()), None)
        }
        State::FinWait2 if has_fin => to_timewait(e, hdr, now_ms),
        State::Closing if acks_fin => {
            e.tcb.state = State::TimeWait;
            (RxAction::None, Some(now_ms + crate::tcp::msl_2_ms()))
        }
        State::LastAck if acks_fin => (RxAction::Reap(e.handle), None),
        State::TimeWait if has_fin => to_timewait(e, hdr, now_ms),
        _ => (RxAction::None, None),
    }
}

fn to_timewait(e: &mut Entry, hdr: &TcpHeader, now_ms: u64) -> (RxAction, Option<u64>) {
    e.tcb.recv.nxt = hdr.seq.wrapping_add(1);
    e.tcb.state = State::TimeWait;
    (RxAction::Reply(e.tcb, FLAG_ACK, Vec::new()), Some(now_ms + crate::tcp::msl_2_ms()))
}
