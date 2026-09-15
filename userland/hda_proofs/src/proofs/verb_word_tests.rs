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

//! Packing a codec verb, which is arithmetic and needs no window.
//!
//! A verb is one 32-bit word carrying the codec address, the node, the command
//! and its payload in fixed fields. Every field is silently truncated on the
//! way out, so a command aimed at the wrong node is accepted by the ring,
//! answered by whatever node it did reach, and reported as a success. There is
//! no error path to catch this on hardware, which is why it is checked here.

use crate::constants::{
    PARAM_VENDOR_ID, POWER_D0, STREAM_FMT_48K16S, VERB_GET_PARAMETER, VERB_SET_POWER_STATE,
    VERB_SET_STREAM_FORMAT,
};
use crate::controller::{compose_verb, compose_verb_long};

#[test]
fn each_field_lands_where_the_specification_puts_it() {
    // Codec at 31:28, node at 26:20, a twelve-bit verb at 19:8, payload at 7:0.
    assert_eq!(compose_verb(0, 0, VERB_GET_PARAMETER, PARAM_VENDOR_ID), 0x000f_0000);
    assert_eq!(compose_verb(1, 2, VERB_SET_POWER_STATE, POWER_D0 as u16), 0x1027_0500);
    assert_eq!(compose_verb(0xf, 0x7f, 0xfff, 0xff), 0xf7ff_ffff);
}

#[test]
fn the_long_form_trades_eight_verb_bits_for_eight_payload_bits() {
    /*
     * A four-bit verb at 19:16 and a sixteen-bit payload at 15:0. The node and
     * codec fields do not move, so both forms address the same widget.
     */
    let fmt = compose_verb_long(0, 2, VERB_SET_STREAM_FORMAT as u16, STREAM_FMT_48K16S);
    assert_eq!(fmt, 0x0022_0011);
    assert_eq!(compose_verb_long(1, 2, 0xf, 0xffff), 0x102f_ffff);
}

#[test]
fn an_out_of_range_address_is_masked_rather_than_bleeding_into_its_neighbour() {
    /*
     * Node 0x80 does not exist. Letting bit 7 through would set bit 27, which
     * is reserved, and a reserved bit set is a command the codec may ignore.
     */
    assert_eq!(compose_verb(0x10, 0, 0, 0), 0);
    assert_eq!(compose_verb(0, 0x80, 0, 0), 0);
    assert_eq!(compose_verb(0, 0, 0x1000, 0), 0);
    assert_eq!(compose_verb_long(0, 0, 0x10, 0), 0);
}
