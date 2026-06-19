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

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut};

use crate::error::{AhciError, AhciResult};

pub struct DmaRegion {
    grant_id: u64,
    user_va: u64,
    device_addr: u64,
}

impl DmaRegion {
    pub fn map(device_id: u64, claim_epoch: u64, length: u64) -> AhciResult<Self> {
        let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
        let r = mk_dma_map(device_id, claim_epoch, length, 0, &mut out);
        if r < 0 {
            return Err(AhciError::BrokerCallFailed(r));
        }
        Ok(Self { grant_id: out.grant_id, user_va: out.user_va, device_addr: out.device_addr })
    }

    pub const fn user_va(&self) -> u64 {
        self.user_va
    }

    pub const fn device_addr(&self) -> u64 {
        self.device_addr
    }
}

impl Drop for DmaRegion {
    fn drop(&mut self) {
        let _ = mk_dma_unmap(self.grant_id);
    }
}
