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

//! The legacy transport. A legacy part dictates its queue size and the rings
//! must sit where the legacy layout puts them for that size, which the
//! driver's fixed region only does at 256 entries.

use crate::constants::{
    LEG_GUEST_FEATURES, LEG_QUEUE_PFN, LEG_STATUS, STATUS_DRIVER_OK, STATUS_FAILED,
    VIRTIO_GPU_F_EDID, VIRTIO_GPU_F_VIRGL, VIRTIO_GPU_TRANSITIONAL, VQ_AVAIL_OFFSET, VQ_MAX_SIZE,
    VQ_USED_OFFSET,
};
use crate::init::bring_up;
use crate::regs::Regs;
use crate::tests::model::{legacy_window, REGION_PHYS};

/// The status byte as the virtio specification numbers it: ACKNOWLEDGE 1,
/// DRIVER 2, DRIVER_OK 4, FEATURES_OK 8.
const LIVE: u8 = 0x0F;

/// Where the legacy layout puts the rings for `size` entries: the available
/// ring directly after the descriptor table, the used ring on the next page.
fn legacy_layout(size: usize) -> (usize, usize) {
    let avail = 16 * size;
    let used = (avail + 6 + 2 * size).div_ceil(4096) * 4096;
    (avail, used)
}

#[test]
fn the_fixed_region_matches_the_legacy_layout_at_the_size_it_was_built_for() {
    assert_eq!(legacy_layout(VQ_MAX_SIZE as usize), (VQ_AVAIL_OFFSET, VQ_USED_OFFSET));
    assert_ne!(legacy_layout(64), (VQ_AVAIL_OFFSET, VQ_USED_OFFSET), "and at no other");
}

#[test]
fn a_legacy_part_gets_only_edid_acked_and_its_page_frame() {
    let bar = legacy_window(VIRTIO_GPU_F_VIRGL | VIRTIO_GPU_F_EDID | 1 << 7, VQ_MAX_SIZE);
    let out = bring_up(Regs::mmio(bar.base()), REGION_PHYS, VIRTIO_GPU_TRANSITIONAL).expect("live");
    assert_eq!(bar.wrote8(LEG_STATUS), LIVE);
    assert_eq!(bar.wrote32(LEG_GUEST_FEATURES), VIRTIO_GPU_F_EDID, "3D has no legacy form");
    assert_eq!(bar.wrote32(LEG_QUEUE_PFN), (REGION_PHYS >> 12) as u32);
    assert!(out.edid && !out.virgl);
    assert_eq!(out.queue_size, VQ_MAX_SIZE);
}

#[test]
fn a_legacy_part_whose_rings_would_not_sit_in_the_fixed_region_is_refused() {
    let bar = legacy_window(VIRTIO_GPU_F_EDID, 64);
    let err = bring_up(Regs::mmio(bar.base()), REGION_PHYS, VIRTIO_GPU_TRANSITIONAL).err();
    assert_eq!(err, Some("virtio-gpu: legacy queue size does not fit the ring layout"));
    assert_ne!(bar.wrote8(LEG_STATUS) & STATUS_FAILED, 0);
    assert_eq!(bar.wrote8(LEG_STATUS) & STATUS_DRIVER_OK, 0);
    assert_eq!(bar.wrote32(LEG_QUEUE_PFN), 0, "no ring was handed over");
}

#[test]
fn a_legacy_part_without_a_queue_is_marked_failed() {
    let bar = legacy_window(VIRTIO_GPU_F_EDID, 0);
    let err = bring_up(Regs::mmio(bar.base()), REGION_PHYS, VIRTIO_GPU_TRANSITIONAL).err();
    assert_eq!(err, Some("virtio-gpu: missing control queue"));
    assert_ne!(bar.wrote8(LEG_STATUS) & STATUS_FAILED, 0);
}
