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
use super::ids::{
    PROBE_HEIGHT, PROBE_PIXEL, PROBE_RESOURCE_ID, PROBE_WIDTH, RENDER_CTX_ID,
};
use crate::device::cmd::{transfer_from_host_3d, Box3d};
use crate::device::ControlQueue;
use crate::state::FenceCounter;

/// Pull the rendered surface back and check the host actually wrote the clear
/// colour. An acknowledged command stream that produced no pixels fails here.
pub fn readback(
    q: &ControlQueue,
    fences: &FenceCounter,
    user_va: u64,
) -> Result<(), &'static str> {
    let region = Box3d::whole_2d(PROBE_WIDTH, PROBE_HEIGHT);
    transfer_from_host_3d(q, RENDER_CTX_ID, fences.issue(), PROBE_RESOURCE_ID, &region, 0)?;

    // SAFETY: eK@nonos.systems - user_va is the start of the DMA region the
    // kernel granted for this probe, at least PROBE_BYTES long, and the
    // transfer above completed before this read.
    let first = unsafe { core::slice::from_raw_parts(user_va as *const u8, 4) };
    if first != PROBE_PIXEL {
        return Err("virtio-gpu: readback pixel does not match the clear colour");
    }
    Ok(())
}
