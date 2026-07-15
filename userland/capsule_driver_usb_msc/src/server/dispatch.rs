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

use crate::protocol::*;
use crate::server::{handlers, respond};
use crate::state::State;

pub(super) fn dispatch(state: &mut State, sender: u32, req: Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender, &req, tx),
        OP_PROBE_CONFIG => handlers::probe_config::handle(state, sender, &req, body, tx),
        OP_BUILD_INQUIRY if body.is_empty() => {
            handlers::build_inquiry::handle(state, sender, &req, tx)
        }
        OP_BUILD_READ_CAPACITY10 if body.is_empty() => {
            handlers::build_capacity::handle(state, sender, &req, tx)
        }
        OP_BUILD_READ10 => handlers::build_read::handle(state, sender, &req, body, tx),
        OP_BUILD_WRITE10 => handlers::build_write::handle(state, sender, &req, body, tx),
        OP_ACCEPT_CSW => handlers::accept_csw::handle(state, sender, &req, body, tx),
        OP_BUILD_TEST_UNIT_READY if body.is_empty() => {
            handlers::build_tur::handle(state, sender, &req, tx)
        }
        OP_BUILD_REQUEST_SENSE => {
            handlers::build_request_sense::handle(state, sender, &req, body, tx)
        }
        OP_DECODE_INQUIRY => handlers::decode_inquiry::handle(sender, &req, body, tx),
        OP_DECODE_CAPACITY => handlers::decode_capacity::handle(sender, &req, body, tx),
        OP_DECODE_SENSE => handlers::decode_sense::handle(sender, &req, body, tx),
        OP_GET_STATE if body.is_empty() => handlers::get_state::handle(state, sender, &req, tx),
        _ if body.is_empty() => {
            let _ = respond::status(sender, &req, E_BAD_OP, tx);
        }
        _ => {
            let _ = respond::status(sender, &req, E_INVAL, tx);
        }
    }
}
