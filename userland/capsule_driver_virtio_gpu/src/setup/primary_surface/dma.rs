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
use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_HIGH};
const PAGE_SIZE: u64 = 4096;
const PAGE_MASK: u64 = PAGE_SIZE - 1;
pub fn map(device_id: u64, claim_epoch: u64, byte_len: u64) -> Result<DmaMapOut, &'static str> {
    let map_len =
        byte_len.checked_add(PAGE_MASK).ok_or("virtio-gpu: surface map length overflow")?
            & !PAGE_MASK;
    let mut dma = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let rc = mk_dma_map(device_id, claim_epoch, map_len, MK_DMA_MAP_HIGH, &mut dma);
    if rc < 0 || dma.user_va == 0 || dma.device_addr == 0 {
        return Err("virtio-gpu: primary dma map failed");
    }
    Ok(dma)
}
pub fn rollback(grant_id: u64, label: &'static str) -> Result<(), &'static str> {
    if mk_dma_unmap(grant_id) < 0 {
        return Err(label);
    }
    Ok(())
}
