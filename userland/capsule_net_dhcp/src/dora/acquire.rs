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

use crate::dhcp::{Message, State as ClientState};
use crate::state::{Lease, STATE};

use super::{discover, install, request, DiscoverError, RequestError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcquireError {
    NoLink,
    Timeout,
    Nak,
}

pub fn acquire() -> Result<(), AcquireError> {
    let (l2, ip) = (STATE.l2(), STATE.ip());
    let mac = *STATE.mac.lock();
    if l2 == 0 || ip == 0 || mac == [0; 6] {
        return Err(AcquireError::NoLink);
    }
    let xid = STATE.next_xid().ok_or(AcquireError::NoLink)?;
    let msg = Message::new_request(&mac, xid);
    *STATE.client_state.lock() = ClientState::Selecting;
    let offer = discover(l2, &msg).map_err(|e| reset(map_discover(e)))?;
    *STATE.client_state.lock() = ClientState::Requesting;
    let ack = request(l2, &msg, &offer).map_err(|e| reset(map_request(e)))?;
    let prefix = install(ip, &ack).map_err(|_| reset(AcquireError::NoLink))?;
    let _ = crate::l2_client::set_ip(l2, ack.yiaddr);
    *STATE.lease.lock() = Lease {
        ipv4: ack.yiaddr,
        prefix,
        gateway: ack.router,
        server_id: ack.server_id,
        dns: ack.dns,
        lease_seconds: ack.lease_seconds,
    };
    *STATE.client_state.lock() = ClientState::Bound;
    Ok(())
}

fn reset(err: AcquireError) -> AcquireError {
    *STATE.client_state.lock() = ClientState::Init;
    err
}

fn map_discover(err: DiscoverError) -> AcquireError {
    match err {
        DiscoverError::Wait(_) => AcquireError::Timeout,
        DiscoverError::Send(_) => AcquireError::NoLink,
    }
}

fn map_request(err: RequestError) -> AcquireError {
    match err {
        RequestError::Nak => AcquireError::Nak,
        RequestError::Wait(_) => AcquireError::Timeout,
        RequestError::Send(_) => AcquireError::NoLink,
    }
}
