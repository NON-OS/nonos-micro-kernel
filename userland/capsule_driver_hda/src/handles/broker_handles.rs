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

pub struct BrokerHandles {
    pub(super) device_id: u64,
    pub(super) mmio_grant_id: u64,
    pub(super) mmio_user_va: u64,
    pub(super) irq_grant_id: u64,
    pub(super) corb_grant_id: u64,
    pub(super) rirb_grant_id: u64,
}

impl BrokerHandles {
    pub(crate) const fn new(
        device_id: u64,
        mmio_grant_id: u64,
        mmio_user_va: u64,
        irq_grant_id: u64,
        corb_grant_id: u64,
        rirb_grant_id: u64,
    ) -> Self {
        Self {
            device_id,
            mmio_grant_id,
            mmio_user_va,
            irq_grant_id,
            corb_grant_id,
            rirb_grant_id,
        }
    }
}
