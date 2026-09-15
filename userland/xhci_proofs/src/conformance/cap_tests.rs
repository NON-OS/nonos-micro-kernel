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

//! Reading the capability registers a controller describes itself with.

use super::model::{controller, CAP_LEN, DOORBELL, RUNTIME};
use crate::constants::{HCCPARAMS1, HCSPARAMS1, HCSPARAMS2};
use crate::controller::refuse_unsupported;
use crate::error::XhciError;
use crate::regs::cap::{
    caplength, context_size, dboff, max_ports, max_scratchpad, max_slots, rtsoff,
};

#[test]
fn the_layout_fields_come_from_where_the_specification_puts_them() {
    let bar = controller(0);
    assert_eq!(caplength(bar.base()) as usize, CAP_LEN);
    assert_eq!(max_slots(bar.base()), 8);
    assert_eq!(max_ports(bar.base()), 2);
    assert_eq!(max_scratchpad(bar.base()), 4);
    assert_eq!(dboff(bar.base()) as usize, DOORBELL);
    assert_eq!(rtsoff(bar.base()) as usize, RUNTIME);
    assert_eq!(context_size(bar.base()), 32);
}

#[test]
fn the_scratchpad_count_assembles_its_high_and_low_fields() {
    let bar = controller(0);
    bar.present32(HCSPARAMS2 as usize, (0x1F << 21) | (0x1 << 27));
    assert_eq!(max_scratchpad(bar.base()), (0x1F << 5) | 0x1);
}

#[test]
fn offsets_ignore_their_reserved_low_bits() {
    let bar = controller(0);
    bar.present32(0x14, 0x1803);
    bar.present32(0x18, 0x101F);
    assert_eq!(dboff(bar.base()), 0x1800);
    assert_eq!(rtsoff(bar.base()), 0x1000);
}

#[test]
fn sixty_four_byte_contexts_are_read_from_csz() {
    let bar = controller(0);
    bar.present32(HCCPARAMS1 as usize, 0x1 | (1 << 2));
    assert_eq!(context_size(bar.base()), 64);
}

#[test]
fn a_controller_without_64_bit_addressing_or_slots_is_refused() {
    let bar = controller(0);
    refuse_unsupported(bar.base()).expect("the modelled controller is supported");
    bar.present32(HCCPARAMS1 as usize, 0);
    assert_eq!(refuse_unsupported(bar.base()), Err(XhciError::ControllerUnsupported));
    bar.present32(HCCPARAMS1 as usize, 0x1);
    bar.present32(HCSPARAMS1 as usize, 2 << 24);
    assert_eq!(refuse_unsupported(bar.base()), Err(XhciError::ControllerUnsupported));
}
