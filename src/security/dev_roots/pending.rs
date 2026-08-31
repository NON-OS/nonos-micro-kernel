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

use spin::Mutex;

/// A root waiting for the user to confirm it.
///
/// Holding one at a time is deliberate. A queue of pending enrolments invites
/// a caller to flood it and hope the user confirms the wrong one, and there is
/// no legitimate reason to enrol two authorities in the same breath.
pub(super) struct Pending {
    pub root: [u8; 32],
    pub challenge: u32,
    pub live: bool,
}

impl Pending {
    const fn new() -> Self {
        Self { root: [0u8; 32], challenge: 0, live: false }
    }

    pub fn arm(&mut self, root: [u8; 32], challenge: u32) {
        self.root = root;
        self.challenge = challenge;
        self.live = true;
    }

    /// Take the pending root if `answer` matches, and clear it either way.
    ///
    /// Cleared on a wrong answer as well as a right one, so a caller gets one
    /// attempt per request rather than being able to sit and guess. Six digits
    /// is a million possibilities, which is ample against one try and nothing
    /// at all against a million.
    pub fn redeem(&mut self, answer: u32) -> Option<[u8; 32]> {
        if !self.live {
            return None;
        }
        let correct = self.challenge == answer;
        let root = self.root;
        self.live = false;
        self.challenge = 0;
        self.root = [0u8; 32];
        if correct {
            Some(root)
        } else {
            None
        }
    }
}

pub(super) static PENDING: Mutex<Pending> = Mutex::new(Pending::new());
