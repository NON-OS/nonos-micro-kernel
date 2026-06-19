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

use nonos_libc::{mk_device_release, mk_mmio_unmap};

pub struct BrokerHandles {
    device_id: u64,
    reg_grant_id: u64,
    fb_grant_id: u64,
}

impl BrokerHandles {
    pub const fn new(device_id: u64, reg_grant_id: u64, fb_grant_id: u64) -> Self {
        Self { device_id, reg_grant_id, fb_grant_id }
    }
}

impl Drop for BrokerHandles {
    fn drop(&mut self) {
        let _ = mk_mmio_unmap(self.fb_grant_id);
        let _ = mk_mmio_unmap(self.reg_grant_id);
        let _ = mk_device_release(self.device_id);
    }
}
