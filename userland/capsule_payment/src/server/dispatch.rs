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
use crate::protocol::{encode_response, Request, EINVAL, OP_HEALTHCHECK, OP_PAY};
use crate::store::State;

pub fn dispatch(state: &mut State, req: Request<'_>) -> Vec<u8> {
    match req.op {
        OP_HEALTHCHECK => handlers::health(req),
        OP_PAY => handlers::pay(state, req),
        _ => encode_response(req.seq, EINVAL, &[]),
    }
}
