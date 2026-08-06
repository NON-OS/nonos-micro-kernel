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
//! HTTP/1.1 for a client, with no I/O of its own.
//!
//! Requests are built into bytes and responses are parsed from bytes, so the
//! same code runs over TLS in the shell and over a buffer in a test. Nothing
//! here opens a socket.

#![no_std]

extern crate alloc;

mod error;
mod request;
mod response;
mod stream;

pub use error::HttpError;
pub use request::{Request, RequestBuilder};
pub use response::{parse_response, Response};
pub use stream::{fetch, Stream};
