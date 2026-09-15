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

//! The modern bring-up against parts that cannot be driven, which must be
//! left marked FAILED and never DRIVER_OK.

use crate::constants::{
    MOD_DEVICE_STATUS, MOD_QUEUE_ENABLE, STATUS_DRIVER_OK, STATUS_FAILED, VIRTIO_GPU_F_VIRGL,
    VIRTIO_GPU_MODERN,
};
use crate::init::bring_up;
use crate::tests::model::{modern_regs, modern_window, QUEUE_SIZE, REGION_PHYS};

#[test]
fn a_part_without_version_1_is_marked_failed_and_refused() {
    let bar = modern_window(0, QUEUE_SIZE);
    let err = bring_up(modern_regs(&bar), REGION_PHYS, VIRTIO_GPU_MODERN).err();
    assert_eq!(err, Some("virtio-gpu: modern feature missing"));
    let status = bar.wrote8(MOD_DEVICE_STATUS);
    assert_ne!(status & STATUS_FAILED, 0, "the part is told");
    assert_eq!(status & STATUS_DRIVER_OK, 0, "and never told the driver is ready");
    assert_eq!(bar.wrote16(MOD_QUEUE_ENABLE), 0, "no queue was enabled");
}

#[test]
fn a_part_without_a_control_queue_is_marked_failed_and_refused() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, 0);
    let err = bring_up(modern_regs(&bar), REGION_PHYS, VIRTIO_GPU_MODERN).err();
    assert_eq!(err, Some("virtio-gpu: missing control queue"));
    let status = bar.wrote8(MOD_DEVICE_STATUS);
    assert_ne!(status & STATUS_FAILED, 0);
    assert_eq!(status & STATUS_DRIVER_OK, 0);
    assert_eq!(bar.wrote16(MOD_QUEUE_ENABLE), 0);
}
