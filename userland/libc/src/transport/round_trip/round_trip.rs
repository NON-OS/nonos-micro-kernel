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
use crate::transport::error::TransportError;
use crate::transport::seq::Counter;

use super::parse_response::parse_response;
use super::prepare_request::prepare_request;
use super::recv_response::recv_response;
use super::send_request::send_request;
use super::types::{Response, RoundTrip};

pub fn round_trip<'a, 'b>(
    counter: &Counter,
    req: RoundTrip<'a>,
    out_buf: &'b mut [u8],
    scratch: &mut [u8],
) -> Result<Response<'b>, TransportError> {
    let request_id = counter.fetch();
    let total = prepare_request(&req, request_id, scratch)?;
    send_request(req.target_port, scratch, total)?;
    let n = recv_response(out_buf, req.timeout_ms)?;
    parse_response(&req, out_buf, n, request_id)
}
