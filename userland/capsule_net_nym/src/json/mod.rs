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

//! Just enough JSON for the gateway's control frames.

mod byte_array;
mod escape;
mod find_key;
mod number;
mod read_number;
mod read_string;
mod writer;

pub use find_key::find_key;
pub use read_number::{read_bytes, read_u64};
pub use read_string::read_string;
pub use writer::tagged_bytes_request;
