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

//! The modern bring-up: what the part is told about its rings.

use crate::constants::{
    MOD_QUEUE_DESC, MOD_QUEUE_DEVICE, MOD_QUEUE_DRIVER, MOD_QUEUE_ENABLE, MOD_QUEUE_SELECT,
    MOD_QUEUE_SIZE, VIRTIO_GPU_F_VIRGL, VIRTIO_GPU_MODERN, VQ_AVAIL_OFFSET, VQ_MAX_SIZE,
    VQ_USED_OFFSET,
};
use crate::init::bring_up;
use crate::tests::model::{
    modern_regs, modern_window, NOTIFY_AT, NOTIFY_MULT, NOTIFY_OFF, QUEUE_SIZE, REGION_PHYS,
};

#[test]
fn the_rings_are_named_at_the_layout_the_queue_code_uses_and_the_queue_is_enabled() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, QUEUE_SIZE);
    let out = bring_up(modern_regs(&bar), REGION_PHYS, VIRTIO_GPU_MODERN).expect("live");
    assert_eq!(bar.wrote16(MOD_QUEUE_SELECT), 0, "the control queue");
    assert_eq!(bar.wrote16(MOD_QUEUE_SIZE), QUEUE_SIZE);
    assert_eq!(out.queue_size, QUEUE_SIZE);
    let desc = bar.wrote32(MOD_QUEUE_DESC) as u64 | (bar.wrote32(MOD_QUEUE_DESC + 4) as u64) << 32;
    let avail =
        bar.wrote32(MOD_QUEUE_DRIVER) as u64 | (bar.wrote32(MOD_QUEUE_DRIVER + 4) as u64) << 32;
    let used =
        bar.wrote32(MOD_QUEUE_DEVICE) as u64 | (bar.wrote32(MOD_QUEUE_DEVICE + 4) as u64) << 32;
    assert_eq!(desc, REGION_PHYS);
    assert_eq!(avail, REGION_PHYS + VQ_AVAIL_OFFSET as u64);
    assert_eq!(used, REGION_PHYS + VQ_USED_OFFSET as u64);
    assert_eq!(bar.wrote16(MOD_QUEUE_ENABLE), 1);
}

#[test]
fn a_queue_larger_than_the_region_holds_is_cut_to_what_fits() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, 1024);
    let out = bring_up(modern_regs(&bar), REGION_PHYS, VIRTIO_GPU_MODERN).expect("live");
    assert_eq!(out.queue_size, VQ_MAX_SIZE);
    assert_eq!(bar.wrote16(MOD_QUEUE_SIZE), VQ_MAX_SIZE, "the part was told the cut size");
}

#[test]
fn a_notification_lands_at_the_offset_the_part_named_scaled_by_the_multiplier() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, QUEUE_SIZE);
    let out = bring_up(modern_regs(&bar), REGION_PHYS, VIRTIO_GPU_MODERN).expect("live");
    let at = NOTIFY_AT + NOTIFY_OFF as usize * NOTIFY_MULT;
    bar.present16(at, 0xFFFF);
    unsafe { out.regs.notify(0) };
    assert_eq!(bar.wrote16(at), 0, "queue zero, at the doorbell for queue zero");
}
