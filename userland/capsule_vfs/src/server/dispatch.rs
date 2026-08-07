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

use super::handlers;
use crate::protocol::{
    encode_response, Request, EINVAL, OP_CHMOD, OP_CLOSE, OP_COPY, OP_HEALTHCHECK, OP_LIST,
    OP_MKDIR, OP_OPEN, OP_READ, OP_RENAME, OP_RMDIR, OP_SEEK, OP_STAT, OP_STORE_PERSIST,
    OP_STORE_REMOVE, OP_STORE_STATUS, OP_TRUNCATE, OP_UNLINK, OP_USAGE, OP_WRITE,
};
use crate::store::Store;

pub fn dispatch(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    match req.op {
        OP_OPEN => handlers::open(store, req, sender_pid),
        OP_CLOSE => handlers::close(store, req, sender_pid),
        OP_READ => handlers::read(store, req, sender_pid),
        OP_WRITE => handlers::write(store, req, sender_pid),
        OP_STAT => handlers::stat(store, req, sender_pid),
        OP_LIST => handlers::list(store, req, sender_pid),
        OP_MKDIR => handlers::mkdir(store, req, sender_pid),
        OP_UNLINK => handlers::unlink(store, req, sender_pid),
        OP_RENAME => handlers::rename(store, req, sender_pid),
        OP_RMDIR => handlers::rmdir(store, req, sender_pid),
        OP_COPY => handlers::copy(store, req, sender_pid),
        OP_TRUNCATE => handlers::truncate(store, req, sender_pid),
        OP_SEEK => handlers::seek(store, req, sender_pid),
        OP_STORE_PERSIST => handlers::store_persist(store, req, sender_pid),
        OP_STORE_REMOVE => handlers::store_remove(req, sender_pid),
        OP_STORE_STATUS => handlers::store_status(req),
        OP_USAGE => handlers::usage(store, req, sender_pid),
        OP_CHMOD => handlers::chmod(store, req, sender_pid),
        OP_HEALTHCHECK => handlers::healthcheck(req),
        _ => encode_response(req.op, req.flags, req.request_id, EINVAL, &[]),
    }
}
