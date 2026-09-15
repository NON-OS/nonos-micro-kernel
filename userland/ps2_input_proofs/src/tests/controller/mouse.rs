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

//! What sits behind the aux port: a mouse, nothing, or a port that echoes.

use crate::constants::{INTELLIMOUSE_ID, MOUSE_ACK, MOUSE_GET_DEVICE_ID, MOUSE_SET_SAMPLE_RATE};

const PLAIN_ID: u8 = 0x00;
const WHEEL_KNOCK: [u8; 3] = [200, 100, 80];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MouseKind {
    /// Nothing answers.
    Absent,
    /// A firmware-emulated port with no device: every byte comes back as
    /// aux data.
    Echoing,
    /// A mouse; `wheel` says whether it knows the IntelliMouse knock.
    Present { wheel: bool },
}

pub struct Mouse {
    pub kind: MouseKind,
    knock: [u8; 3],
    expecting_rate: bool,
    pub reporting: bool,
}

impl Mouse {
    pub fn new(kind: MouseKind) -> Self {
        Self { kind, knock: [0; 3], expecting_rate: false, reporting: false }
    }

    pub fn command(&mut self, value: u8) -> Vec<u8> {
        let wheel = match self.kind {
            MouseKind::Absent => return Vec::new(),
            MouseKind::Echoing => return vec![value],
            MouseKind::Present { wheel } => wheel,
        };
        if self.expecting_rate {
            self.expecting_rate = false;
            self.knock = [self.knock[1], self.knock[2], value];
            return vec![MOUSE_ACK];
        }
        match value {
            MOUSE_SET_SAMPLE_RATE => {
                self.expecting_rate = true;
                vec![MOUSE_ACK]
            }
            MOUSE_GET_DEVICE_ID if wheel && self.knock == WHEEL_KNOCK => {
                vec![MOUSE_ACK, INTELLIMOUSE_ID]
            }
            MOUSE_GET_DEVICE_ID => vec![MOUSE_ACK, PLAIN_ID],
            0xF4 => {
                self.reporting = true;
                vec![MOUSE_ACK]
            }
            _ => vec![MOUSE_ACK],
        }
    }
}
