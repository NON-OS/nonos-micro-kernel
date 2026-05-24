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
use nonos_libc::{mk_dma_map, DmaMapOut};
use super::page::{MAX_PAGES_PER_GRANT, PAGE_SIZE};
use super::pool::DmaPool;
use super::region::DmaRegion;
use crate::error::{XhciError, XhciResult};
impl DmaPool {
    pub fn alloc(&self, requested: u64) -> XhciResult<DmaRegion> {
        if requested == 0 {
            return Err(XhciError::ControllerUnsupported);
        }
        let length = (requested + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        if length / PAGE_SIZE > MAX_PAGES_PER_GRANT {
            return Err(XhciError::ControllerUnsupported);
        }
        let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
        let r = mk_dma_map(self.device_id, self.claim_epoch, length, 0, &mut out);
        if r < 0 {
            return Err(XhciError::BrokerCallFailed(r));
        }
        Ok(DmaRegion {
            user_va: out.user_va,
            device_addr: out.device_addr,
            length: out.length,
            grant_id: out.grant_id,
        })
    }
}