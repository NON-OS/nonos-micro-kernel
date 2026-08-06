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

//! The message a SOCKS5 connection carries through the mixnet to a network
//! requester, the exit that makes the real TCP connection and returns data
//! through single-use reply blocks. This is Nym's `Socks5Request` and
//! `Socks5Response` wire format, protocol version 3, reimplemented so a live
//! exit reads our traffic:
//!
//!   request  = [version 3][flag][body]
//!     Connect (flag 0): conn_id(u64 be) | addr_len(u16 be) | "host:port"
//!     Send    (flag 1): conn_id(u64 be) | local_closed(1) | seq(u64 be) | data
//!   response = [version 3][flag][body]
//!     NetworkData     (flag 1): conn_id | local_closed | seq | data
//!     ConnectionError (flag 2): conn_id | utf8 message
//!
//! The connect request carries no return address: replies come back through
//! reply blocks the mixnet client attaches when it sends, so the exit never
//! learns who asked. The `seq` numbers let both ends reassemble a stream the
//! mixnet may reorder. Every encoder is bounds checked and every decoder
//! rejects a short or unknown message rather than trusting it.

mod constants;
mod decode;
mod encode;
mod hostport;
mod provider;
mod writer;

pub use constants::{
    PROTOCOL_VERSION, REQ_CONNECT, REQ_SEND, RESP_CONNECTION_ERROR, RESP_NETWORK_DATA,
};
pub use decode::{decode_response, Response};
pub use encode::{encode_connect, encode_send};
pub use hostport::write_hostport;
pub use provider::{ENVELOPE_BYTES, INTERFACE_VERSION, TAG_PROVIDER_DATA};
