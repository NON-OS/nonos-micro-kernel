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

//! Opening what comes back: a reply arrives on one of the blocks we handed
//! out, sealed under that block's key.

mod assembly;
mod message;
mod open;
mod reassemble;
mod types;

pub use assembly::Assembly;
pub use message::{reply_body, reply_message, Reply};
pub use open::open_reply;
pub use reassemble::collect;
pub use types::{
    DIGEST_BYTES, RECIPIENT_BYTES, TAG_REPLY_DATA, TAG_REPLY_SURB_REQUEST, TYPE_REPLY,
};
