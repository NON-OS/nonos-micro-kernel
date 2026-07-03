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

use super::bit::bit;
use crate::constants::{
    LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_STATUS, STATUS_ACKNOWLEDGE, STATUS_DRIVER,
    STATUS_FAILED, STATUS_FEATURES_OK, VIRTIO_NET_F_MAC, VIRTIO_NET_F_STATUS,
};
use crate::regs::Regs;

pub fn negotiate(regs: Regs) -> Result<u32, &'static str> {
    unsafe {
        regs.w8(LEG_STATUS, 0);
        regs.w8(LEG_STATUS, STATUS_ACKNOWLEDGE);
        regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER);
        let host = regs.r32(LEG_HOST_FEATURES);
        let want = host & (bit(VIRTIO_NET_F_MAC) | bit(VIRTIO_NET_F_STATUS));
        regs.w32(LEG_GUEST_FEATURES, want);
        let s = regs.r8(LEG_STATUS);
        regs.w8(LEG_STATUS, s | STATUS_FEATURES_OK);
        let s2 = regs.r8(LEG_STATUS);
        if s2 & STATUS_FEATURES_OK == 0 {
            regs.w8(LEG_STATUS, s2 | STATUS_FAILED);
            return Err("virtio-net: features-ok rejected");
        }
        Ok(want)
    }
}
