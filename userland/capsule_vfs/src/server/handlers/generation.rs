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

//! Answer with the store's generation counter.

use alloc::vec::Vec;

use crate::protocol::{encode_response, Request, OP_GENERATION};
use crate::server::generation;

/// Eight bytes and no walk. This exists so a caller polling for change does not
/// have to list a directory to find out there was none.
pub fn generation(req: Request<'_>) -> Vec<u8> {
    let n = generation::current();
    encode_response(OP_GENERATION, req.flags, req.request_id, 0, &n.to_le_bytes())
}
