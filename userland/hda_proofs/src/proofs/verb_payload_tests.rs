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

//! The one verb whose payload does not fit the short form.
//!
//! Split from `verb_word_tests` because it is the case with a consequence on
//! hardware rather than a field-placement check: send the amplifier unmute
//! through the eight-bit form and the codec answers, the driver moves on, and
//! nothing is ever unmuted.

use crate::constants::{AMP_OUT_UNMUTE, VERB_SET_AMP_GAIN_MUTE};
use crate::controller::{compose_verb, compose_verb_long};

const AMP_GAIN: u16 = 0x7f;

#[test]
fn an_amplifier_payload_only_survives_the_form_that_is_wide_enough_for_it() {
    /*
     * The unmute payload sets the output-amp select bits in the high byte and
     * the gain in the low one. Sent through the eight-bit form the select bits
     * are dropped, which programs a gain into no amplifier at all: the codec
     * answers, the driver moves on, and nothing is ever unmuted.
     */
    let payload = AMP_OUT_UNMUTE | AMP_GAIN;
    let long = compose_verb_long(0, 2, VERB_SET_AMP_GAIN_MUTE as u16, payload);
    assert_eq!(long & 0xffff, payload as u32, "the wide form must carry the payload whole");
    let short = compose_verb(0, 2, VERB_SET_AMP_GAIN_MUTE as u16, payload);
    assert_eq!(short & 0xff, AMP_GAIN as u32, "only the gain survives the eight-bit form");
    assert_eq!(short & AMP_OUT_UNMUTE as u32, 0, "the amplifier select bits are dropped");
}
