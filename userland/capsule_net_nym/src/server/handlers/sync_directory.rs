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

use crate::directory_sync;
use crate::protocol::{
    E_DIRECTORY_PROTO, E_DIRECTORY_SOURCE, E_NO_TCP, E_OK, E_PERM, OP_SYNC_DIRECTORY,
};
use crate::server::authz::admin;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::TABLE;
use crate::{setup, topology};

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !admin(pid) {
        return respond(pid, OP_SYNC_DIRECTORY, E_PERM, req.request_id, 0, tx);
    }
    let tcp = setup::tcp_port();
    if tcp == 0 {
        return respond(pid, OP_SYNC_DIRECTORY, E_NO_TCP, req.request_id, 0, tx);
    }
    // No source named means the public API. A caller asking for it takes one
    // step of the fetch, the same step the idle tick takes, because the whole
    // directory is more than one round trip and holding the capsule for all
    // of them costs every other client its reply.
    if body.is_empty() && crate::state::directory_source().is_none() {
        let errno = match directory_sync::sync_step(tcp) {
            directory_sync::Step::Done(_) | directory_sync::Step::Progressed => E_OK,
            directory_sync::Step::Failed(E_NO_TCP) => E_NO_TCP,
            directory_sync::Step::Failed(_) => E_DIRECTORY_PROTO,
        };
        return respond(pid, OP_SYNC_DIRECTORY, errno, req.request_id, 0, tx);
    }
    let source = match source(body) {
        Ok(source) => source,
        Err(e) => return respond(pid, OP_SYNC_DIRECTORY, e, req.request_id, 0, tx),
    };
    let errno = match directory_sync::fetch(tcp, &source) {
        Ok(dir) => install(&dir),
        Err(E_NO_TCP) => E_NO_TCP,
        Err(_) => E_DIRECTORY_PROTO,
    };
    respond(pid, OP_SYNC_DIRECTORY, errno, req.request_id, 0, tx);
}

fn source(body: &[u8]) -> Result<directory_sync::DirectorySource, u16> {
    if body.is_empty() {
        return crate::state::directory_source().ok_or(E_DIRECTORY_SOURCE);
    }
    let source = directory_sync::parse(body)?;
    crate::state::install_directory_source(source.clone());
    Ok(source)
}

fn install(dir: &[u8]) -> u16 {
    match topology::install(dir) {
        Ok(()) => {
            TABLE.lock().reset_sessions();
            E_OK
        }
        Err(e) => super::topology_errno::map(e),
    }
}
