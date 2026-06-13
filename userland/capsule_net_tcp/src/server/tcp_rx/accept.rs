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

use crate::state::TABLE;
use crate::tcp::{Endpoint4, State, Tcb};

pub fn syn(local: Endpoint4, remote: Endpoint4, seq: u32) -> Option<Tcb> {
    let mut table = TABLE.lock();
    let (owner, parent) = {
        let l = table.listener_for_mut(local.port)?;
        (l.owner_pid, l.handle)
    };
    let iss = table.iss_for_pair(local, remote);
    let mut tcb = Tcb::listen(local);
    tcb.remote = remote;
    tcb.state = State::SynReceived;
    tcb.recv.irs = seq;
    tcb.recv.nxt = seq.wrapping_add(1);
    tcb.recv.wnd = 8192;
    tcb.send.iss = iss;
    tcb.send.nxt = iss;
    tcb.send.wnd = 8192;
    let tx_tcb = tcb;
    tcb.send.nxt = iss.wrapping_add(1);
    let _ = table.insert(owner, parent, tcb).ok()?;
    Some(tx_tcb)
}
