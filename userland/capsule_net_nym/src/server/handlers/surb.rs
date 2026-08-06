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

use crate::protocol::{
    E_AUTHORITY_MISSING, E_AUTHORITY_UNTRUSTED, E_BAD_LEN, E_CREDENTIAL_EXPIRED, E_CRYPTO,
    E_NO_CREDENTIAL, E_NO_SESSION, E_OK, OP_CREATE_SURB,
};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::{self, CredentialError, TABLE};

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != 4 && body.len() != 12 {
        return respond(pid, OP_CREATE_SURB, E_BAD_LEN, req.request_id, 0, tx);
    }
    let session = match u32_at(body, 0) {
        Ok(session) => session,
        Err(e) => return respond(pid, OP_CREATE_SURB, e, req.request_id, 0, tx),
    };
    if !TABLE.lock().has_session(pid, session) {
        return respond(pid, OP_CREATE_SURB, E_NO_SESSION, req.request_id, 0, tx);
    }
    let cred = match state::credential_material() {
        Ok(cred) => cred,
        Err(CredentialError::Expired) => {
            return respond(pid, OP_CREATE_SURB, E_CREDENTIAL_EXPIRED, req.request_id, 0, tx);
        }
        Err(CredentialError::NoAuthority) => {
            return respond(pid, OP_CREATE_SURB, E_AUTHORITY_MISSING, req.request_id, 0, tx);
        }
        Err(CredentialError::UntrustedAuthority) => {
            return respond(pid, OP_CREATE_SURB, E_AUTHORITY_UNTRUSTED, req.request_id, 0, tx);
        }
        Err(_) => return respond(pid, OP_CREATE_SURB, E_NO_CREDENTIAL, req.request_id, 0, tx),
    };
    let Some((id, tag)) = state::create_surb(pid, session, &cred, ttl_ms(body)) else {
        return respond(pid, OP_CREATE_SURB, E_CRYPTO, req.request_id, 0, tx);
    };
    tx[20..24].copy_from_slice(&id.to_le_bytes());
    tx[24..56].copy_from_slice(&tag);
    respond(pid, OP_CREATE_SURB, E_OK, req.request_id, 36, tx);
}

fn ttl_ms(body: &[u8]) -> u64 {
    if body.len() < 12 {
        return state::surb_ttl_ms();
    }
    u64::from_le_bytes([body[4], body[5], body[6], body[7], body[8], body[9], body[10], body[11]])
}
