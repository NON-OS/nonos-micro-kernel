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

use super::super::consts::{DRAIN_BATCH_MAX, RECORD_LEN};
use crate::protocol::{encode_response, Request};
use crate::store::State;

pub fn drain(state: &mut State, req: Request<'_>) -> Vec<u8> {
    let batch = state.take_batch(DRAIN_BATCH_MAX);
    let mut payload = Vec::with_capacity(4 + batch.len() * RECORD_LEN);
    payload.extend_from_slice(&(batch.len() as u32).to_le_bytes());
    for record in &batch {
        payload.extend_from_slice(record);
    }
    encode_response(req.seq, 0, &payload)
}
