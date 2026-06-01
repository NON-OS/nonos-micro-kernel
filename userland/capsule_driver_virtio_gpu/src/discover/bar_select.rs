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

use nonos_libc::{DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO};

use crate::constants::VIRTIO_GPU_MODERN;

const MODERN_CFG_MIN: u64 = 0x4000;
const MODERN_CFG_MAX: u64 = 0x1_0000;

pub fn select(r: &DeviceRecord) -> Option<(u8, u8, u64)> {
    if r.device == VIRTIO_GPU_MODERN {
        return modern_mmio(r).or_else(|| first_of_kind(r, BAR_KIND_MMIO));
    }
    first_of_kind(r, BAR_KIND_PIO).or_else(|| first_of_kind(r, BAR_KIND_MMIO))
}

fn modern_mmio(r: &DeviceRecord) -> Option<(u8, u8, u64)> {
    let mut best: Option<(u8, u8, u64)> = None;
    for i in 0..r.bars.len() {
        let bar = r.bars[i];
        if bar.kind != BAR_KIND_MMIO || bar.size < MODERN_CFG_MIN || bar.size > MODERN_CFG_MAX {
            continue;
        }
        best = match best {
            Some((_, _, size)) if size <= bar.size => best,
            _ => Some((i as u8, bar.kind, bar.size)),
        };
    }
    best
}

fn first_of_kind(r: &DeviceRecord, kind: u8) -> Option<(u8, u8, u64)> {
    for i in 0..r.bars.len() {
        let bar = r.bars[i];
        if bar.kind == kind && bar.size != 0 {
            return Some((i as u8, bar.kind, bar.size));
        }
    }
    None
}
