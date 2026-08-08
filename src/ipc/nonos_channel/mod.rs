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

//! Wire envelope (`IpcMessage`) and MAC-key bootstrap (`init_ipc_secret`).
//! Capsule traffic enqueues directly on `nonos_inbox`; this module owns
//! only the on-the-wire format and integrity primitives.

mod error;
mod hash;
mod limits;
mod message;

pub use error::ChannelError;
pub use hash::{compute_channel_key, compute_checksum, init_ipc_secret};
pub use limits::MAX_MESSAGE_SIZE;
pub use message::IpcMessage;
