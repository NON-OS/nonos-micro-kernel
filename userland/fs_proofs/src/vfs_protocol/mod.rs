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

// The vfs wire protocol codec, from capsule source. `decode_request` parses
// attacker-controlled bytes, so its bounds handling is proven against hostile
// input.

#[path = "../../../capsule_vfs/src/protocol/decode.rs"]
mod decode;
#[path = "../../../capsule_vfs/src/protocol/encode.rs"]
mod encode;
#[path = "../../../capsule_vfs/src/protocol/errno.rs"]
mod errno;
#[path = "../../../capsule_vfs/src/protocol/types.rs"]
mod types;

pub use decode::{decode_request, DecodeError};
pub use encode::encode_response;
// Glob so every protocol constant defined in the real headers is part of this
// crate's surface rather than flagged unused.
pub use errno::*;
pub use types::*;
