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

//! The request queue: where the part is told it is, and how big.

use crate::constants::{LEG_QUEUE_PFN, LEG_QUEUE_SEL};
use crate::init::bring_up;
use crate::regs::Regs;

use super::model::device;

#[test]
fn the_queue_address_is_written_as_a_page_number_not_a_byte_address() {
    /*
     * The legacy queue register takes a page frame number. Writing the byte
     * address instead is the classic way to point a device at memory 4096
     * times further up than intended, and it is invisible until the device
     * does a DMA. `0x1234_5000 >> 12` is the number that belongs there.
     */
    let bar = device(64);
    bring_up(Regs::mmio(bar.base()), 0x1234_5000, 64).expect("bring-up");
    assert_eq!(bar.wrote32(LEG_QUEUE_PFN), 0x0001_2345);
}

#[test]
fn the_queue_is_selected_before_its_size_is_believed() {
    /*
     * Queue 0 is the requestq. Reading the size without selecting first reads
     * whichever queue the device happened to have selected.
     */
    let bar = device(64);
    bring_up(Regs::mmio(bar.base()), 0x1000, 64).expect("bring-up");
    assert_eq!(bar.wrote16(LEG_QUEUE_SEL), 0);
}

#[test]
fn the_queue_size_is_clamped_to_what_the_device_offers() {
    /*
     * Asking for more than the device has is the driver's error to absorb, not
     * the device's to police.
     */
    let bar = device(16);
    let size = bring_up(Regs::mmio(bar.base()), 0x1000, 64).expect("bring-up");
    assert_eq!(size, 16);
}

#[test]
fn a_generous_device_does_not_widen_the_queue_past_the_request() {
    let bar = device(1024);
    let size = bring_up(Regs::mmio(bar.base()), 0x1000, 64).expect("bring-up");
    assert_eq!(size, 64);
}
