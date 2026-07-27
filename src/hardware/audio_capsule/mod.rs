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

//! Kernel-side glue for the userland audio.server capsule.
//! The kernel embeds and spawns the signed IPC-only capsule; all
//! mixing and PCM forwarding stays inside `audio.server`, which owns
//! no hardware and holds only IPC | Memory | Debug.

mod embed;
mod spawn;
mod state;

pub use spawn::{spawn_audio_capsule, SpawnError};
pub use state::shared_state;
