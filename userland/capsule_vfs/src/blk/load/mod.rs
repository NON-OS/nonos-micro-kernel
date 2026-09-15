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

//! Reading the package store without going deaf while it happens.
//!
//! `store::load` reads the whole container in one call: a capacity query, two
//! header reads, then for every entry an extent walked in whole-sector chunks
//! because payloads run to hundreds of kilobytes against a driver that caps a
//! request, plus a digest over each payload. That is potentially hundreds of
//! block round trips.
//!
//! vfs_pool ran it from the idle slot of its receive loop, so for the whole of
//! that it answered nobody. The seeder's own comment said the cost was "up to
//! one blk timeout per call"; it is one per chunk. Every caller that polled
//! through the window spent a full IPC timeout per attempt, which is why the
//! desktop shell logged forty dead calls a boot and why three separate clients
//! grew backoffs to work around it.
//!
//! This does the same work, resumable, on a time budget. The loop hands control
//! back to the receive path after at most a few milliseconds, so the longest
//! anyone waits is one block request rather than the whole store.

mod begin;
mod step;
mod types;

pub use types::{Load, Step};
