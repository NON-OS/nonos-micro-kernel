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

use super::types::Rte;

impl Rte {
    pub fn to_u32s(self) -> (u32, u32) {
        // The IO-APIC RTE only has an 8-bit destination field, so a
        // BSP APIC ID above 255 (possible in x2APIC mode) would silently
        // target the wrong CPU once masked to 0xFF. We keep the mask but
        // fail loudly instead of quietly, since it must never go unseen.
        if self.dest_apic_id > 0xFF {
            crate::sys::serial::println(
                b"[IOAPIC] warning: dest APIC id > 0xFF truncated to 8 bits (x2APIC)",
            );
        }
        let mut low = self.vector as u32;
        low |= (self.delivery as u32) << 8;
        if self.logical {
            low |= 1 << 11;
        }
        if self.active_low {
            low |= 1 << 13;
        }
        if self.level_trigger {
            low |= 1 << 15;
        }
        if self.masked {
            low |= 1 << 16;
        }
        let high = (self.dest_apic_id & 0xFF) << 24;
        (low, high)
    }

    pub fn from_u32s(low: u32, high: u32) -> Self {
        Self {
            vector: (low & 0xFF) as u8,
            delivery: ((low >> 8) & 0x7) as u8,
            logical: (low & (1 << 11)) != 0,
            active_low: (low & (1 << 13)) != 0,
            level_trigger: (low & (1 << 15)) != 0,
            masked: (low & (1 << 16)) != 0,
            dest_apic_id: (high >> 24) & 0xFF,
        }
    }
}
