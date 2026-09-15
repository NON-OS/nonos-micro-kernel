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

//! Finding out which codecs are actually there.
//!
//! The immediate command interface is the only way to talk to a codec before
//! the rings are up, and it is another exchange memory cannot fake: the driver
//! raises a busy bit and waits for the controller to drop it and raise a valid
//! one. Which codec answers decides which widgets the capsule goes looking
//! for, so a probe that reports a codec at the wrong address sends the whole
//! output path to a node on a part that is not listening.

use nonos_devmodel::run;

use crate::constants::{IC, IR, PARAM_VENDOR_ID, VERB_GET_PARAMETER};
use crate::controller::{compose_verb, probe, MAX_CODECS};
use crate::model::{immediate_responder, window};
use crate::regs::Regs;

/// A Realtek ALC269, the codec on most of the laptops this capsule targets.
pub const VENDOR_ID: u32 = 0x10ec_0269;

#[test]
fn a_present_codec_is_identified_over_the_immediate_command_interface() {
    let bar = window();
    bar.present32(IR as usize, VENDOR_ID);
    let _controller = run(&bar, immediate_responder);
    let found = probe(Regs::new(bar.base()), 0b1);
    assert_eq!((found[0].present, found[0].ok), (1, 1), "codec zero never answered");
    assert_eq!(found[0].vendor_id, 0x10ec, "the vendor is the upper half of the response");
    assert_eq!(found[0].device_id, 0x0269, "the device is the lower half");
}

#[test]
fn the_verb_that_asks_for_the_identity_names_the_codec_that_was_asked() {
    /*
     * The address is a field of the command word, not a separate register.
     * Composing it against the wrong codec reads a second part's identity and
     * reports it at the first part's address.
     */
    let bar = window();
    bar.present32(IR as usize, VENDOR_ID);
    let _controller = run(&bar, immediate_responder);
    let _ = probe(Regs::new(bar.base()), 1 << 3);
    let expected = compose_verb(3, 0, VERB_GET_PARAMETER, PARAM_VENDOR_ID);
    assert_eq!(bar.wrote32(IC as usize), expected);
}

#[test]
fn an_absent_codec_is_left_alone_and_still_carries_its_own_address() {
    /*
     * The capsule reports this array straight out to its clients, so an entry
     * that says nothing about which address it describes is worse than absent.
     */
    let bar = window();
    let found = probe(Regs::new(bar.base()), 0);
    for (i, p) in found.iter().enumerate() {
        assert_eq!(p.address as usize, i, "slot {i} describes another address");
        assert_eq!((p.present, p.ok, p.vendor_id), (0, 0, 0), "slot {i} invented a codec");
    }
    assert_eq!(found.len(), MAX_CODECS);
}
