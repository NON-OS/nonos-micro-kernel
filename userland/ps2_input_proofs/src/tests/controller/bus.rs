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

//! The register interface: what a status read shows, what a data read
//! yields, and where each write goes.

use super::state::Controller;
use crate::constants::{STATUS_AUX_DATA, STATUS_INPUT_FULL, STATUS_OFFSET, STATUS_OUTPUT_FULL};

impl Controller {
    /// A status read is the clock here: each one brings delayed bytes a
    /// step closer, and delivers those whose time is up.
    fn tick(&mut self) {
        let mut i = 0;
        while i < self.delayed.len() {
            if self.delayed[i].0 == 0 {
                let (_, byte, aux) = self.delayed.remove(i);
                self.output.push_back((byte, aux));
            } else {
                self.delayed[i].0 -= 1;
                i += 1;
            }
        }
    }

    pub fn read(&mut self, offset: u16) -> u8 {
        if offset != STATUS_OFFSET {
            return self.output.pop_front().map_or(0, |(byte, _)| byte);
        }
        self.tick();
        let mut status = 0;
        if let Some(&(_, aux)) = self.output.front() {
            status |= STATUS_OUTPUT_FULL;
            if aux {
                status |= STATUS_AUX_DATA;
            }
        }
        if self.input_stuck {
            status |= STATUS_INPUT_FULL;
        }
        status
    }

    pub fn write(&mut self, offset: u16, value: u8) {
        self.writes.push((offset, value));
        if offset == STATUS_OFFSET {
            self.command(value);
        } else {
            self.data(value);
        }
    }
}
