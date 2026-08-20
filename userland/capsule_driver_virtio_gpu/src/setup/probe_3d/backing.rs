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
use super::ids::{PROBE_BYTES, PROBE_RESOURCE_ID};
use crate::device::cmd::attach_backing;
use crate::device::ControlQueue;
use crate::state::FenceCounter;
use nonos_libc::{mk_dma_map, DmaMapOut};

/// Guest pages the host reads back into. Without these the render is only
/// provable by the host's own acknowledgement.
pub fn map(
    q: &ControlQueue,
    fences: &FenceCounter,
    device_id: u64,
    claim_epoch: u64,
) -> Result<DmaMapOut, &'static str> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(device_id, claim_epoch, PROBE_BYTES, 0, &mut out) < 0 {
        return Err("virtio-gpu: probe dma map failed");
    }
    if out.user_va == 0 || out.device_addr == 0 || out.length < PROBE_BYTES {
        return Err("virtio-gpu: probe dma region too small");
    }
    attach_backing(
        q,
        fences.issue(),
        PROBE_RESOURCE_ID,
        out.device_addr,
        PROBE_BYTES as u32,
    )?;
    Ok(out)
}
