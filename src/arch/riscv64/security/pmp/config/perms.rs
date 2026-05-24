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

use super::{PmpAddressMode, PmpConfig};
use super::super::constants::{PMP_A_NA4, PMP_A_NAPOT, PMP_A_OFF, PMP_A_TOR};
use super::super::constants::{PMP_L, PMP_R, PMP_W, PMP_X};

impl PmpConfig {
    pub(super) fn perms(&self) -> u8 {
        let read = if self.read { PMP_R } else { 0 };
        let write = if self.write { PMP_W } else { 0 };
        let execute = if self.execute { PMP_X } else { 0 };
        read | write | execute
    }

    pub(super) fn mode_bits(&self) -> u8 {
        match self.address_mode {
            PmpAddressMode::Off => PMP_A_OFF,
            PmpAddressMode::Tor => PMP_A_TOR,
            PmpAddressMode::Na4 => PMP_A_NA4,
            PmpAddressMode::Napot => PMP_A_NAPOT,
        }
    }

    pub(super) fn lock_bits(&self) -> u8 {
        if self.locked { PMP_L } else { 0 }
    }
}
