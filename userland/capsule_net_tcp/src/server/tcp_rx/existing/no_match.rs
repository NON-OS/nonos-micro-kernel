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

use crate::server::tcp_rx::action::RxAction;
use crate::tcp::{Endpoint4, TcpHeader, FLAG_ACK, FLAG_RST, FLAG_SYN};

pub fn no_match(local: Endpoint4, remote: Endpoint4, hdr: &TcpHeader, has_listener: bool) -> RxAction {
    if hdr.has_flag(FLAG_RST) {
        return RxAction::None;
    }
    if hdr.has_flag(FLAG_SYN) && !hdr.has_flag(FLAG_ACK) && has_listener {
        return RxAction::None;
    }
    if hdr.has_flag(FLAG_ACK) {
        return RxAction::Rst { local, remote, seq: hdr.ack, ack: 0 };
    }
    RxAction::Rst { local, remote, seq: 0, ack: hdr.seq.wrapping_add(1) }
}
