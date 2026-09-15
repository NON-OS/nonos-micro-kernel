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

//! What each command does to the controller, and where a data byte goes.

use super::state::{Controller, Pending, CONFIG_KBD_DISABLE};
use crate::constants::{
    CONFIG_AUX_DISABLE, CTL_DISABLE_AUX, CTL_ENABLE_AUX, CTL_ENABLE_KBD, CTL_READ_CONFIG,
    CTL_WRITE_AUX, CTL_WRITE_CONFIG,
};

impl Controller {
    pub(super) fn command(&mut self, cmd: u8) {
        self.pending = Pending::Nothing;
        match cmd {
            CTL_READ_CONFIG => self.output.push_back((self.config, false)),
            CTL_WRITE_CONFIG => self.pending = Pending::ConfigWrite,
            CTL_ENABLE_AUX => {
                self.config &= !CONFIG_AUX_DISABLE;
                self.aux_clock = true;
            }
            CTL_DISABLE_AUX => {
                self.config |= CONFIG_AUX_DISABLE;
                self.aux_clock = false;
            }
            CTL_ENABLE_KBD => self.config &= !CONFIG_KBD_DISABLE,
            CTL_WRITE_AUX => self.pending = Pending::AuxWrite,
            _ => {}
        }
    }

    /// A data byte goes where the last command said: to the configuration,
    /// to the mouse if its clock is on, or to the keyboard if its clock is.
    pub(super) fn data(&mut self, value: u8) {
        match core::mem::replace(&mut self.pending, Pending::Nothing) {
            Pending::ConfigWrite => self.config = value,
            Pending::AuxWrite if self.aux_clock => {
                for byte in self.mouse.command(value) {
                    self.output.push_back((byte, true));
                }
            }
            Pending::AuxWrite => {}
            Pending::Nothing if self.config & CONFIG_KBD_DISABLE == 0 => {
                for (delay, byte) in self.keyboard.command(value) {
                    self.delayed.push((delay, byte, false));
                }
            }
            Pending::Nothing => {}
        }
    }
}
