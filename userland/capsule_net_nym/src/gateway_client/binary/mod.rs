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

//! Encrypted binary frames between client and gateway.

mod blob;
mod kinds;
mod parse;
mod response;

pub use blob::make_encrypted_blob;
pub use kinds::{KIND_FORWARD_SPHINX, KIND_FORWARD_SPHINX_V2};
pub use parse::{parse_blob, Incoming};
pub use response::{is_pushed_message, KIND_PUSHED_MIX_MESSAGE};
