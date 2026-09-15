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

//! A keyboard: acknowledges what it is sent, some status reads later.

use crate::constants::MOUSE_ACK;

const KBD_RESET: u8 = 0xFF;
const SELF_TEST_PASSED: u8 = 0xAA;

pub struct Keyboard {
    /// Status reads between a command and its acknowledgement; `None` is a
    /// keyboard that never answers.
    pub latency: Option<u32>,
}

impl Keyboard {
    pub const PROMPT: Self = Self { latency: Some(0) };
    pub const SLOW: Self = Self { latency: Some(3) };
    pub const DEAD: Self = Self { latency: None };

    /// The bytes a command draws, each with the delay before it appears.
    pub fn command(&self, value: u8) -> Vec<(u32, u8)> {
        let Some(latency) = self.latency else {
            return Vec::new();
        };
        match value {
            KBD_RESET => vec![(latency, MOUSE_ACK), (latency + 2, SELF_TEST_PASSED)],
            _ => vec![(latency, MOUSE_ACK)],
        }
    }
}
