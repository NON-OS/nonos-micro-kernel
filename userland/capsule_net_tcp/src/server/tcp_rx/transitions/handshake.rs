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
use crate::tcp::{State, TcpHeader, FLAG_ACK, FLAG_SYN};

pub fn step(e: &mut Entry, hdr: &TcpHeader) -> RxAction {
    if hdr.flags & (FLAG_SYN | FLAG_ACK) == FLAG_SYN | FLAG_ACK {
        e.tcb.recv.nxt = hdr.seq.wrapping_add(1);
        e.tcb.send.una = hdr.ack;
        e.tcb.state = State::Established;
        return RxAction::Reply(e.tcb, FLAG_ACK, Vec::new());
    }
    RxAction::None
}
