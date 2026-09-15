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

//! The controller's registers and what it remembers.

use std::collections::VecDeque;

use super::{Keyboard, Mouse};
use crate::constants::STATUS_OFFSET;

/// Configuration byte bit 4: the first port's clock is held off.
pub const CONFIG_KBD_DISABLE: u8 = 1 << 4;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Pending {
    Nothing,
    ConfigWrite,
    AuxWrite,
}

pub struct Controller {
    pub config: u8,
    /// Bytes waiting in the output buffer, each with its aux flag.
    pub output: VecDeque<(u8, bool)>,
    /// Every write the driver made, as (offset, byte), in order.
    pub writes: Vec<(u16, u8)>,
    /// A controller whose input buffer never drains.
    pub input_stuck: bool,
    pub keyboard: Keyboard,
    pub mouse: Mouse,
    pub(super) pending: Pending,
    pub(super) aux_clock: bool,
    /// Bytes on their way to the output buffer: status reads to go, byte,
    /// aux flag.
    pub(super) delayed: Vec<(u32, u8, bool)>,
}

impl Controller {
    pub fn new(config: u8, keyboard: Keyboard, mouse: Mouse) -> Self {
        Self {
            config,
            output: VecDeque::new(),
            writes: Vec::new(),
            input_stuck: false,
            keyboard,
            mouse,
            pending: Pending::Nothing,
            aux_clock: false,
            delayed: Vec::new(),
        }
    }

    /// The bytes written to the command port, in order.
    pub fn commands(&self) -> Vec<u8> {
        self.writes.iter().filter(|w| w.0 == STATUS_OFFSET).map(|w| w.1).collect()
    }

    /// The bytes written to the data port, in order.
    pub fn data_writes(&self) -> Vec<u8> {
        self.writes.iter().filter(|w| w.0 != STATUS_OFFSET).map(|w| w.1).collect()
    }
}
