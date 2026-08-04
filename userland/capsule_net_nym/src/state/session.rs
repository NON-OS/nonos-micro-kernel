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

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::crypto::Key;
use crate::packet::REPLAY_TAG_LEN;

use super::gateway::Gateway;
use super::replay::ReplayWindow;

pub const RX_DEPTH: usize = 8;

pub struct Session {
    pub owner: u32,
    pub id: u32,
    pub gateway: Gateway,
    pub key: Key,
    /// Zero until set. Without one there is nowhere to route, so the session
    /// cannot be sealed as Sphinx.
    pub dest: [u8; 32],
    /// The exit's x25519 key. A message is sealed for the exit alone under a
    /// key agreed against this, so the identity above cannot stand in for it:
    /// one names the destination, the other encrypts to it.
    pub dest_encryption: [u8; 32],
    pub dest_id: [u8; 16],
    replay: ReplayWindow,
    rx: VecDeque<Vec<u8>>,
}

impl Session {
    pub fn new(owner: u32, id: u32, gateway: Gateway, key: Key) -> Self {
        Self {
            owner,
            id,
            gateway,
            key,
            dest: [0u8; 32],
            dest_encryption: [0u8; 32],
            dest_id: [0u8; 16],
            replay: ReplayWindow::new(),
            rx: VecDeque::new(),
        }
    }

    pub fn push(&mut self, body: Vec<u8>) {
        if self.rx.len() == RX_DEPTH && self.rx.pop_front().is_none() {
            return;
        }
        self.rx.push_back(body);
    }

    pub fn pop(&mut self) -> Option<Vec<u8>> {
        self.rx.pop_front()
    }

    pub fn accept_replay_tag(&mut self, tag: &[u8; REPLAY_TAG_LEN]) -> bool {
        self.replay.accept(tag)
    }

    pub fn zeroize(&mut self) {
        self.key.fill(0);
    }
}
