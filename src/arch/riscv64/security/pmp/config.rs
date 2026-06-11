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

mod perms;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmpAddressMode {
    Off,
    Tor,
    Na4,
    Napot,
}

#[derive(Debug, Clone, Copy)]
pub struct PmpConfig {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub address_mode: PmpAddressMode,
    pub locked: bool,
}

impl PmpConfig {
    pub const fn new() -> Self {
        Self {
            read: false,
            write: false,
            execute: false,
            address_mode: PmpAddressMode::Off,
            locked: false,
        }
    }

    pub const fn rwx() -> Self {
        Self {
            read: true,
            write: true,
            execute: true,
            address_mode: PmpAddressMode::Napot,
            locked: false,
        }
    }

    pub const fn ro() -> Self {
        Self {
            read: true,
            write: false,
            execute: false,
            address_mode: PmpAddressMode::Napot,
            locked: false,
        }
    }

    pub const fn rx() -> Self {
        Self {
            read: true,
            write: false,
            execute: true,
            address_mode: PmpAddressMode::Napot,
            locked: false,
        }
    }

    pub const fn rw() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
            address_mode: PmpAddressMode::Napot,
            locked: false,
        }
    }

    pub fn to_cfg_byte(&self) -> u8 {
        self.perms() | self.mode_bits() | self.lock_bits()
    }
}

impl Default for PmpConfig {
    fn default() -> Self {
        Self::new()
    }
}
