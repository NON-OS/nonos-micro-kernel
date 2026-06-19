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

use crate::protocol::{encode_response, Request, OP_HEALTHCHECK};

// HealthCheck: zero-input, zero-output. Reaching this handler proves
// the request decoder accepted the envelope and the dispatcher routed
// the op. Used by validation checks as a structural liveness probe.
pub fn healthcheck(req: Request<'_>) -> Vec<u8> {
    encode_response(OP_HEALTHCHECK, req.flags, req.request_id, 0, &[])
}
