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

//! Register encodings the specification fixes and a typo would break.

use super::model::{controller, op_base, wrote64, CAP_LEN, RUNTIME};
use crate::constants::{
    CONFIG, CRCR_LO, DCBAAP_LO, ERDP_LO, ERSTSZ, IMAN, IMAN_IE, INTERRUPTER_STRIDE,
};
use crate::regs::op::{config_set_max_slots, crcr_program, dcbaap_program};
use crate::regs::runtime::{erdp_program, erstsz_program, iman_read, iman_write, interrupter_addr};

#[test]
fn the_command_ring_pointer_keeps_its_alignment_bits_clear_and_carries_rcs() {
    let bar = controller(0);
    crcr_program(op_base(&bar), 0x0000_0001_2345_6FC0 | 0x3F, 1);
    assert_eq!(wrote64(&bar, CAP_LEN + CRCR_LO as usize), 0x0000_0001_2345_6FC0 | 1);
}

#[test]
fn the_dcbaa_pointer_is_64_byte_aligned_across_both_halves() {
    let bar = controller(0);
    dcbaap_program(op_base(&bar), 0xFFFF_FFFF_FFFF_FFFF);
    assert_eq!(wrote64(&bar, CAP_LEN + DCBAAP_LO as usize), 0xFFFF_FFFF_FFFF_FFC0);
}

#[test]
fn config_writes_the_slot_count_into_the_low_byte_and_keeps_the_rest() {
    let bar = controller(0);
    bar.present32(CAP_LEN + CONFIG as usize, 0xABCD_EF00);
    config_set_max_slots(op_base(&bar), 8);
    assert_eq!(bar.wrote32(CAP_LEN + CONFIG as usize), 0xABCD_EF08);
}

#[test]
fn the_dequeue_pointer_carries_desi_and_ehb_below_its_alignment() {
    let bar = controller(0);
    let intr = interrupter_addr(bar.base() + RUNTIME as u64, 0);
    erdp_program(intr, 0x1234_5670 | 0xF, true, 5);
    let want = 0x1234_5670 | 5 | (1 << 3);
    assert_eq!(wrote64(&bar, RUNTIME + INTERRUPTER_STRIDE as usize + ERDP_LO as usize), want);
}

#[test]
fn the_primary_interrupter_sits_one_stride_past_the_runtime_base() {
    assert_eq!(interrupter_addr(0x1000, 0), 0x1000 + INTERRUPTER_STRIDE);
    assert_eq!(interrupter_addr(0x1000, 3), 0x1000 + 4 * INTERRUPTER_STRIDE);
}

#[test]
fn segment_count_and_interrupt_enable_land_in_their_registers() {
    let bar = controller(0);
    let intr = interrupter_addr(bar.base() + RUNTIME as u64, 0);
    erstsz_program(intr, 1);
    iman_write(intr, iman_read(intr) | IMAN_IE);
    let ir0 = RUNTIME + INTERRUPTER_STRIDE as usize;
    assert_eq!(bar.wrote32(ir0 + ERSTSZ as usize), 1);
    assert_ne!(bar.wrote32(ir0 + IMAN as usize) & IMAN_IE, 0);
}
