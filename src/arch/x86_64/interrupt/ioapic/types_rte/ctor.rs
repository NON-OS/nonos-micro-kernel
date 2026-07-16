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
    pub const fn fixed(vector: u8, dest_apic_id: u32) -> Self {
        Self {
            vector,
            delivery: 0,
            logical: false,
            active_low: false,
            level_trigger: false,
            masked: true,
            dest_apic_id,
        }
    }

    pub const fn nmi(dest_apic_id: u32) -> Self {
        Self {
            vector: 0,
            delivery: 4,
            logical: false,
            active_low: false,
            level_trigger: false,
            masked: true,
            dest_apic_id,
        }
    }
}
