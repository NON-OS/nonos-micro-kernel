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

//! The modern bring-up: the status handshake and the features taken.

use crate::constants::{
    FEATURE_PAGE_HIGH, MOD_DEVICE_STATUS, MOD_DRIVER_FEATURE, MOD_DRIVER_FEATURE_SELECT,
    VIRTIO_F_VERSION_1_HIGH, VIRTIO_GPU_F_EDID, VIRTIO_GPU_F_VIRGL, VIRTIO_GPU_MODERN,
};
use crate::init::bring_up;
use crate::tests::model::{modern_regs, modern_window, QUEUE_SIZE, REGION_PHYS};

/// The status byte as the virtio specification numbers it: ACKNOWLEDGE 1,
/// DRIVER 2, DRIVER_OK 4, FEATURES_OK 8.
const LIVE: u8 = 0x0F;

#[test]
fn a_part_offering_both_features_ends_live_with_both_accepted() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL | VIRTIO_GPU_F_EDID, QUEUE_SIZE);
    let out = bring_up(modern_regs(&bar), REGION_PHYS, VIRTIO_GPU_MODERN).expect("live");
    assert_eq!(bar.wrote8(MOD_DEVICE_STATUS), LIVE);
    assert!(out.virgl && out.edid);
    assert_eq!(bar.wrote32(MOD_DRIVER_FEATURE_SELECT), FEATURE_PAGE_HIGH, "high page acked last");
    assert_eq!(bar.wrote32(MOD_DRIVER_FEATURE), VIRTIO_F_VERSION_1_HIGH);
}

#[test]
fn a_feature_the_part_did_not_offer_is_not_taken() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, QUEUE_SIZE);
    let out = bring_up(modern_regs(&bar), REGION_PHYS, VIRTIO_GPU_MODERN).expect("live");
    assert!(out.virgl, "offered, so accepted");
    assert!(!out.edid, "not offered, so not claimed");
}
