// NONOS Operating System (AGPL-3.0-or-later)
//! The link predicates gate command issue. A command must not be issued until
//! the PHY reports DET==3 and the device has cleared BSY and DRQ; these pin the
//! exact bit decode against the SATA status and task-file registers.

use crate::engine::link::established::link_established;
use crate::engine::link::ready::device_ready;

#[test]
fn link_is_established_only_when_det_is_three() {
    assert!(link_established(0x3)); // present, PHY communication established
    assert!(link_established(0x0000_0113)); // upper SSTS fields ignored, DET==3
    assert!(!link_established(0x0)); // no device
    assert!(!link_established(0x1)); // present, no communication yet
    assert!(!link_established(0x4)); // PHY offline
}

#[test]
fn device_is_ready_only_when_bsy_and_drq_are_clear() {
    assert!(device_ready(0x00)); // idle
    assert!(device_ready(0x40)); // an unrelated status bit set, BSY/DRQ clear
    assert!(!device_ready(0x80)); // BSY set
    assert!(!device_ready(0x08)); // DRQ set
    assert!(!device_ready(0x88)); // both set
}
