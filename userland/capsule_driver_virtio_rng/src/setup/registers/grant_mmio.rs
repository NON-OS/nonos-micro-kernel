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
use super::types::RegisterGrant;
use crate::discover::Found;
use nonos_libc::{mk_device_release, mk_mmio_map, MmioMapOut};

const PAGE_MASK: u64 = 0xFFF;

pub(super) fn grant_mmio(dev: Found, epoch: u64) -> Result<RegisterGrant, &'static str> {
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let length = (dev.register_size + PAGE_MASK) & !PAGE_MASK;
    let r = mk_mmio_map(dev.device_id, epoch, dev.register_bar as u32, 0, 0, length, &mut out);
    if r < 0 {
        let _ = mk_device_release(dev.device_id);
        return Err("mmio map failed");
    }
    Ok(RegisterGrant::Mmio(out))
}
