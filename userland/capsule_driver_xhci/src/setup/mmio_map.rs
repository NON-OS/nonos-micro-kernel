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
use nonos_libc::{mk_device_release, mk_mmio_map, MmioMapOut};
use crate::error::{XhciError, XhciResult};
const BAR_INDEX: u32 = 0;
pub fn mmio_map(device_id: u64, claim_epoch: u64, bar0_size: u64) -> XhciResult<MmioMapOut> {
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let r = mk_mmio_map(device_id, claim_epoch, BAR_INDEX, 0, 0, bar0_size, &mut out);
    if r < 0 {
        let _ = mk_device_release(device_id);
        return Err(XhciError::BrokerCallFailed(r));
    }
    Ok(out)
}