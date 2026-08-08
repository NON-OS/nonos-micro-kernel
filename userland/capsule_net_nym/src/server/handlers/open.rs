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

use crate::crypto::{fill_random, Key};
use crate::protocol::{
    E_CRYPTO, E_NO_CREDENTIAL, E_NO_GATEWAY, E_NO_TOPOLOGY, E_OK, E_TABLE_FULL, E_TOPOLOGY_EXPIRED,
    OP_OPEN_SESSION,
};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::{TableError, TABLE};

pub fn handle(pid: u32, req: &Request, tx: &mut [u8]) {
    let mut key: Key = [0; 32];
    if fill_random(&mut key).is_err() {
        return respond(pid, OP_OPEN_SESSION, E_CRYPTO, req.request_id, 0, tx);
    }
    let id = match TABLE.lock().open(pid, key) {
        Ok(id) => id,
        Err(TableError::NoGateway) => {
            return respond(pid, OP_OPEN_SESSION, E_NO_GATEWAY, req.request_id, 0, tx);
        }
        Err(TableError::NoTopology) => {
            return respond(pid, OP_OPEN_SESSION, E_NO_TOPOLOGY, req.request_id, 0, tx);
        }
        Err(TableError::NoCredential) => {
            return respond(pid, OP_OPEN_SESSION, E_NO_CREDENTIAL, req.request_id, 0, tx);
        }
        Err(TableError::StaleTopology) => {
            return respond(pid, OP_OPEN_SESSION, E_TOPOLOGY_EXPIRED, req.request_id, 0, tx);
        }
        Err(TableError::Full) => {
            return respond(pid, OP_OPEN_SESSION, E_TABLE_FULL, req.request_id, 0, tx);
        }
    };
    tx[20..24].copy_from_slice(&id.to_le_bytes());
    respond(pid, OP_OPEN_SESSION, E_OK, req.request_id, 4, tx);
}
