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

//! Fetching the directory a piece at a time.
//!
//! The directory is three lists and each is its own TLS handshake and
//! transfer. Doing them in one go holds this capsule for longer than a client
//! is willing to wait for a reply, and a capsule that stops answering is
//! indistinguishable from one that died. One list per call keeps each pause
//! inside what a caller tolerates.

use alloc::vec::Vec;

use crate::topology::Node;
use spin::Mutex;

use super::stages::{exits, first, gateways};

/// What has been gathered so far, and which list comes next.
pub(super) static PARTIAL: Mutex<Option<(Vec<Node>, u8)>> = Mutex::new(None);

/// What one call did.
pub enum Step {
    /// A list arrived; there is more to fetch.
    Progressed,
    /// The directory is installed, with this many nodes.
    Done(usize),
    /// Nothing arrived. The step will be tried again.
    Failed(u16),
}

/// Fetch the next piece of the directory.
pub fn sync_step(tcp_port: u32) -> Step {
    let held = PARTIAL.lock().take();
    match held {
        None => first(tcp_port),
        Some((nodes, 1)) => gateways(tcp_port, nodes),
        Some((nodes, _)) => exits(tcp_port, nodes),
    }
}
