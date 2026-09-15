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

//! The seven verbs that turn a silent codec into an output path.
//!
//! This is the one sequence in the capsule whose order a test can read back
//! exactly, because the ring keeps it: each verb lands in its own numbered
//! slot. Every verb below is accepted, answered and then ignored by a part
//! that was not ready for it, so the only symptom of a wrong sequence is a
//! stream that runs at full rate and makes no sound.

use std::sync::Arc;

use nonos_devmodel::{run, FakeBar};

use crate::constants::{
    AMP_OUT_UNMUTE, PIN_OUT_ENABLE, POWER_D0, STREAM_FMT_48K16S, VERB_SET_AMP_GAIN_MUTE,
    VERB_SET_CHANNEL_STREAMID, VERB_SET_PIN_WIDGET_CONTROL, VERB_SET_POWER_STATE,
    VERB_SET_STREAM_FORMAT,
};
use crate::controller::stream_setup::configure;
use crate::controller::{compose_verb, compose_verb_long, OutputPath};
use crate::model::{corb_engine, rings, window};
use crate::regs::Regs;

pub const CAD: u8 = 0;
pub const AFG: u8 = 1;
pub const DAC: u8 = 2;
pub const PIN: u8 = 3;
pub const TAG: u8 = 1;
pub const GAIN: u16 = 0x7f;

/// The command ring a full output-path configuration was sent through.
pub fn configured() -> Arc<FakeBar> {
    let bar = window();
    let (corb, rirb) = rings(0);
    let _controller = run(&bar, corb_engine);
    let path = OutputPath { dac_nid: DAC, pin_nid: PIN, afg_nid: AFG };
    let mut wp = 0u16;
    let regs = Regs::new(bar.base());
    assert!(configure(regs, corb.base(), rirb.base(), &mut wp, CAD, path, TAG).is_ok());
    assert_eq!(wp, 7, "the path takes seven verbs and no others");
    corb
}

#[test]
fn the_output_path_is_configured_with_exactly_the_verbs_the_codec_needs() {
    let corb = configured();
    let power = |nid| compose_verb(CAD, nid, VERB_SET_POWER_STATE, POWER_D0 as u16);
    let want = [
        power(AFG),
        power(DAC),
        power(PIN),
        compose_verb(CAD, PIN, VERB_SET_PIN_WIDGET_CONTROL, PIN_OUT_ENABLE as u16),
        compose_verb_long(CAD, DAC, VERB_SET_AMP_GAIN_MUTE as u16, AMP_OUT_UNMUTE | GAIN),
        compose_verb_long(CAD, DAC, VERB_SET_STREAM_FORMAT as u16, STREAM_FMT_48K16S),
        compose_verb(CAD, DAC, VERB_SET_CHANNEL_STREAMID, (TAG as u16) << 4),
    ];
    for (i, cmd) in want.iter().enumerate() {
        assert_eq!(corb.wrote32((i + 1) * 4), *cmd, "verb {i} is not the one needed here");
    }
}
