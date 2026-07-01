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
    E_AUTHORITY_MISSING, E_AUTHORITY_UNTRUSTED, E_BAD_LEN, E_CREDENTIAL_EXPIRED, E_CRYPTO, E_OK,
    E_PERM, E_TOPOLOGY_AUTH, OP_SET_CREDENTIAL,
};
use crate::server::authz::admin;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::{self, CredentialError};

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !admin(pid) {
        return respond(pid, OP_SET_CREDENTIAL, E_PERM, req.request_id, 0, tx);
    }
    let errno = match state::install_credential(body) {
        Ok(()) => E_OK,
        Err(CredentialError::BadLength) => E_BAD_LEN,
        Err(CredentialError::BadExpiry) => E_CREDENTIAL_EXPIRED,
        Err(CredentialError::BadSignature) => E_TOPOLOGY_AUTH,
        Err(CredentialError::NoAuthority) => E_AUTHORITY_MISSING,
        Err(CredentialError::UntrustedAuthority) => E_AUTHORITY_UNTRUSTED,
        Err(_) => E_CRYPTO,
    };
    respond(pid, OP_SET_CREDENTIAL, errno, req.request_id, 0, tx);
}
