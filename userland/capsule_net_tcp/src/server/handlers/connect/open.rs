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

use crate::state::{local_ip, next_ephemeral, TABLE};
use crate::tcp::{Endpoint4, State, Tcb};

pub fn connection(owner: u32, dst: [u8; 4], dst_port: u16) -> Option<(u32, Tcb)> {
    let mut table = TABLE.lock();
    let local = Endpoint4 { ip: local_ip(), port: next_ephemeral() };
    let remote = Endpoint4 { ip: dst, port: dst_port };
    let iss = table.iss_for_pair(local, remote);
    let mut tcb = Tcb::listen(local);
    tcb.remote = remote;
    tcb.state = State::SynSent;
    tcb.send.iss = iss;
    tcb.send.nxt = iss;
    tcb.recv.wnd = 8192;
    let tx_tcb = tcb;
    tcb.send.nxt = iss.wrapping_add(1);
    let handle = table.insert(owner, 0, tcb).ok()?;
    Some((handle, tx_tcb))
}
