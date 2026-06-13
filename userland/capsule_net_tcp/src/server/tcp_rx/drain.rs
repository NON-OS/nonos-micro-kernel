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

use crate::ip_client::{poll_segment, RecvError};
use crate::state::ip_port;
use crate::tcp::{parse, Endpoint4, FLAG_ACK, FLAG_RST, FLAG_SYN};

use super::{accept, existing};
use crate::server::tcp_tx;

pub fn drain_one() -> bool {
    let pkt = match poll_segment(ip_port()) {
        Ok(p) => p,
        Err(RecvError::Empty) | Err(_) => return false,
    };
    let Ok((hdr, payload)) = parse(&pkt.src, &pkt.dst, &pkt.segment) else { return true };
    let local = Endpoint4 { ip: pkt.dst, port: hdr.dst_port };
    let remote = Endpoint4 { ip: pkt.src, port: hdr.src_port };
    if hdr.has_flag(FLAG_RST) {
        return true;
    }
    if let Some(plan) = existing::update(local, remote, hdr, payload) {
        let _ = tcp_tx::send(plan.0, plan.1, &plan.2);
        return true;
    }
    if hdr.has_flag(FLAG_SYN) && !hdr.has_flag(FLAG_ACK) {
        if let Some(plan) = accept::syn(local, remote, hdr.seq) {
            let _ = tcp_tx::send(plan, FLAG_SYN | FLAG_ACK, &[]);
        }
    }
    true
}
