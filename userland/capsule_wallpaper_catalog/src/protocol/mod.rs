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

pub mod errno;
pub mod hdr;
pub mod limits;
pub mod ops;

pub use errno::{E_BAD_LEN, E_INVAL, E_NOT_FOUND, E_OK, E_RANGE};
pub use hdr::{Header, HDR_LEN};
pub use limits::{CHUNK_MAX, IPC_PAYLOAD_MAX};
pub use ops::{OP_GET_CHUNK, OP_GET_COUNT, OP_GET_SIZE, OP_GET_SLUG};
