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
//! The wire protocol a fetch speaks.
//!
//! Git frames everything in pkt-lines: four hex digits of length, then that
//! many bytes counting the header itself. Length zero is the flush packet
//! that ends a section.

mod advert;
mod error;
mod pkt;
mod request;
mod update;

pub use advert::{parse_advertisement, RemoteRef};
pub use error::WireError;
pub use pkt::{encode_pkt, read_pkt, Pkt};
pub use request::want_request;
pub use update::{push_request, RefUpdate};
