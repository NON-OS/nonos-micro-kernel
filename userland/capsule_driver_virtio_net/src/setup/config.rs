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

use crate::constants::{LEG_MAC, MAC_LEN, VIRTIO_NET_F_MAC};
use crate::regs::Regs;

pub fn feature_enabled(features: u32, bit: u32) -> bool {
    features & (1u32 << bit) != 0
}

pub fn read_mac(regs: Regs, features: u32) -> [u8; MAC_LEN] {
    let mut mac = [0u8; MAC_LEN];
    if feature_enabled(features, VIRTIO_NET_F_MAC) {
        for (i, b) in mac.iter_mut().enumerate() {
            *b = unsafe { regs.r8(LEG_MAC + i) };
        }
    }
    mac
}
