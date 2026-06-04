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
use crate::constants::VG_MAX_SCANOUTS;
use crate::device::cmd;
use crate::device::virtqueue::ControlQueue;
use crate::state::{FenceCounter, Scanout, ScanoutTable};
const DEFAULT_SCANOUT_WIDTH: u32 = 1024;
const DEFAULT_SCANOUT_HEIGHT: u32 = 768;
pub fn seed(
    q: &ControlQueue,
    table: &ScanoutTable,
    fences: &FenceCounter,
) -> Result<(), &'static str> {
    let info = cmd::get_display_info(q, fences.issue())?;
    let mut recorded = 0usize;
    for i in 0..VG_MAX_SCANOUTS {
        let s = info.scanouts[i];
        if s.enabled == 0 || s.width == 0 || s.height == 0 {
            continue;
        }
        if !table.record(
            i as u32,
            Scanout {
                x: s.x,
                y: s.y,
                width: s.width,
                height: s.height,
                current_resource_id: 0,
                enabled: true,
            },
        ) {
            return Err("virtio-gpu: scanout table rejected display info");
        }
        recorded += 1;
    }
    if recorded == 0 {
        seed_default(table)?;
    }
    Ok(())
}
fn seed_default(table: &ScanoutTable) -> Result<(), &'static str> {
    if table.record(
        0,
        Scanout {
            x: 0,
            y: 0,
            width: DEFAULT_SCANOUT_WIDTH,
            height: DEFAULT_SCANOUT_HEIGHT,
            current_resource_id: 0,
            enabled: true,
        },
    ) {
        return Ok(());
    }
    Err("virtio-gpu: scanout table rejected default mode")
}
