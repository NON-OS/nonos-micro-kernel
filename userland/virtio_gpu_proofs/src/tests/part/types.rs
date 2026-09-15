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

//! How the part answers, what it saw, and the handle a test keeps on it.

use std::sync::{Arc, Mutex};

use nonos_devmodel::LiveDevice;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    Spec,
    Reject,
    WrongDescriptor,
    BadEdidMagic,
    WrongFence,
}

/// One request as the part saw it: the bytes and the flags on each side of
/// the chain.
pub struct Seen {
    pub request: Vec<u8>,
    pub request_flags: u16,
    pub response_flags: u16,
}

pub struct Part {
    pub seen: Arc<Mutex<Vec<Seen>>>,
    _live: LiveDevice,
}

impl Part {
    pub fn new(seen: Arc<Mutex<Vec<Seen>>>, live: LiveDevice) -> Self {
        Self { seen, _live: live }
    }

    pub fn seen(&self) -> Vec<Vec<u8>> {
        self.seen
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .iter()
            .map(|s| s.request.clone())
            .collect()
    }
}
