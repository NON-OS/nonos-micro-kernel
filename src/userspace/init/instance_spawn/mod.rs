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

//! Deferred spawn of extra app-window instances.
//!
//! Spawning a capsule is heavy: it creates an address space, loads and
//! relocates an ELF, mints a capability token, and allocates kernel and
//! user stacks. Boot does all of this from init's context, where there is
//! no live user task waiting to be resumed. Running the same work inline
//! in a live GUI capsule's syscall (the desktop shell asking for a second
//! terminal window) corrupts the caller: the shell resumes under the wrong
//! address space and faults on its own code (cr2 == rip, not present).
//!
//! So `MkSpawnInstance` does not spawn inline. It records a request here
//! and returns, and `init_loop` drains the queue and performs the spawn in
//! init's context, exactly the environment the boot spawns already run in.
//! The window appears a tick later, which is imperceptible and keeps the
//! shell stable.

mod boot_frame;
mod queue;
mod request;
mod service;

pub use queue::PendingApp;
pub use request::request;
pub(crate) use service::service;
