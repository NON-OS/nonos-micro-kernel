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

use super::action::RxAction;
use super::rst;
use super::transitions::{closing, established, handshake};
use crate::state::{TimerKind, TABLE};
use crate::tcp::{Endpoint4, State, TcpHeader, FLAG_ACK, FLAG_FIN, FLAG_RST, FLAG_SYN};

pub fn update(local: Endpoint4, remote: Endpoint4, hdr: TcpHeader, payload: &[u8]) -> RxAction {
    let now = crate::clock::now_ms();
    let mut table = TABLE.lock();
    let mut arm: Option<(u32, u64)> = None;
    let mut accepted: Option<(u32, u32)> = None;
    let action = match table.connection_match_mut(local, remote) {
        None => {
            if hdr.has_flag(FLAG_RST) {
                return RxAction::None;
            }
            if hdr.has_flag(FLAG_SYN)
                && !hdr.has_flag(FLAG_ACK)
                && table.listener_for_mut(local.port).is_some()
            {
                return RxAction::None;
            }
            if hdr.has_flag(FLAG_ACK) {
                return RxAction::Rst { local, remote, seq: hdr.ack, ack: 0 };
            }
            return RxAction::Rst { local, remote, seq: 0, ack: hdr.seq.wrapping_add(1) };
        }
        Some(e) => {
            if hdr.has_flag(FLAG_RST) {
                if rst::in_window(e, hdr.seq) { RxAction::Reap(e.handle) } else { RxAction::None }
            } else if e.tcb.state == State::SynSent {
                handshake::step(e, &hdr)
            } else if e.tcb.state == State::SynReceived
                && hdr.has_flag(FLAG_ACK)
                && hdr.ack == e.tcb.send.nxt
            {
                e.tcb.state = State::Established;
                accepted = Some((e.parent, e.handle));
                if hdr.has_flag(FLAG_FIN) {
                    e.tcb.recv.nxt = e.tcb.recv.nxt.wrapping_add(1);
                    e.tcb.state = State::CloseWait;
                    RxAction::Reply(e.tcb, FLAG_ACK, Vec::new())
                } else {
                    RxAction::None
                }
            } else if e.tcb.state.is_closing() {
                let (a, deadline) = closing::step(e, &hdr, now);
                if let Some(d) = deadline {
                    arm = Some((e.handle, d));
                }
                a
            } else if e.tcb.state.accepts_data() {
                established::step(e, &hdr, payload)
            } else {
                RxAction::None
            }
        }
    };
    if let Some((parent, handle)) = accepted {
        if let Some(p) = table.by_handle_mut(parent) {
            let _ = p.push_accept(handle);
        }
    }
    if let Some((h, d)) = arm {
        table.timers.arm(h, TimerKind::TimeWait, d);
    }
    if let RxAction::Reap(h) = action {
        table.remove_by_handle(h);
        table.timers.cancel_all(h);
        return RxAction::None;
    }
    action
}
