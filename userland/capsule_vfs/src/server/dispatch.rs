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
    encode_response, Request, EINVAL, OP_CLOSE, OP_HEALTHCHECK, OP_LIST, OP_MKDIR, OP_OPEN,
    OP_READ, OP_RENAME, OP_STAT, OP_UNLINK, OP_WRITE,
};
use crate::store::Store;

pub fn dispatch(store: &mut Store, req: Request<'_>) -> Vec<u8> {
    match req.op {
        OP_OPEN => handlers::open(store, req),
        OP_CLOSE => handlers::close(store, req),
        OP_READ => handlers::read(store, req),
        OP_WRITE => handlers::write(store, req),
        OP_STAT => handlers::stat(store, req),
        OP_LIST => handlers::list(store, req),
        OP_MKDIR => handlers::mkdir(store, req),
        OP_UNLINK => handlers::unlink(store, req),
        OP_RENAME => handlers::rename(store, req),
        OP_HEALTHCHECK => handlers::healthcheck(req),
        _ => encode_response(req.op, req.flags, req.request_id, EINVAL, &[]),
    }
}
