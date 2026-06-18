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
use spin::Mutex;

use super::super::error::DriverHdaError;
use super::super::protocol::decode_response;
use super::super::state;
use crate::services::lifecycle::transport;

pub(crate) const REPLY_INBOX: &str = "endpoint.4294967312";
const SENDER_NAME: &str = "kernel.driver_hda";

static TRANSPORT_LOCK: Mutex<()> = Mutex::new(());

pub(super) struct ResponseBytes {
    pub status: i32,
    pub body: Vec<u8>,
}

pub(super) fn round_trip(
    request_id: u32,
    request: Vec<u8>,
) -> Result<ResponseBytes, DriverHdaError> {
    let _guard = transport::lock_yielding(&TRANSPORT_LOCK);
    let resp = transport::round_trip(
        request_id,
        &request,
        SENDER_NAME,
        REPLY_INBOX,
        state::shared_state(),
        decode_response,
    )
    .map_err(map_err)?;
    Ok(ResponseBytes { status: resp.status, body: resp.body })
}

fn map_err(e: transport::TransportError) -> DriverHdaError {
    match e {
        transport::TransportError::Dead => DriverHdaError::Dead,
        transport::TransportError::Stale => DriverHdaError::Stale,
        transport::TransportError::TransportFailure => DriverHdaError::TransportFailure,
        transport::TransportError::ProtocolMismatch => DriverHdaError::ProtocolMismatch,
    }
}
