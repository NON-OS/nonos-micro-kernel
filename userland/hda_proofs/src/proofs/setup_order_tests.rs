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

//! The order those seven verbs have to arrive in.
//!
//! A codec is a tree of widgets that each hold their own state, and a widget
//! in its low power state answers every command and keeps none of them. That
//! is the trap here: the configuration verbs succeed either way, and the
//! difference between a working path and a silent one is only whether the
//! power state verb reached each widget first.

use nonos_devmodel::FakeBar;

use crate::constants::{
    POWER_D0, STREAM_FMT_48K16S, VERB_SET_CHANNEL_STREAMID, VERB_SET_POWER_STATE,
    VERB_SET_STREAM_FORMAT,
};
use crate::controller::{compose_verb, compose_verb_long};
use crate::proofs::setup_tests::{configured, AFG, CAD, DAC, PIN, TAG};

/// The command in ring slot `n`, counting from the first verb sent.
fn slot(corb: &FakeBar, n: usize) -> u32 {
    corb.wrote32((n + 1) * 4)
}

#[test]
fn every_widget_is_powered_up_before_it_is_given_any_other_command() {
    let corb = configured();
    for (i, nid) in [AFG, DAC, PIN].into_iter().enumerate() {
        let want = compose_verb(CAD, nid, VERB_SET_POWER_STATE, POWER_D0 as u16);
        assert_eq!(slot(&corb, i), want, "widget {nid} was configured before it was powered");
    }
}

#[test]
fn the_function_group_is_powered_before_the_widgets_inside_it() {
    /*
     * Powering a converter whose function group is still in D3 is a command
     * to a branch of the tree that is not clocked, so it lands nowhere.
     */
    let corb = configured();
    let afg = compose_verb(CAD, AFG, VERB_SET_POWER_STATE, POWER_D0 as u16);
    assert_eq!(slot(&corb, 0), afg, "a widget was powered ahead of its function group");
}

#[test]
fn the_converter_is_joined_to_a_stream_only_once_it_has_a_format_to_decode() {
    /*
     * The stream identifier is what makes the converter start consuming the
     * link. Setting it before the format leaves it decoding the previous
     * format, or none, for as long as it takes the next verb to arrive.
     */
    let corb = configured();
    let fmt = compose_verb_long(CAD, DAC, VERB_SET_STREAM_FORMAT as u16, STREAM_FMT_48K16S);
    let sid = compose_verb(CAD, DAC, VERB_SET_CHANNEL_STREAMID, (TAG as u16) << 4);
    assert_eq!(slot(&corb, 5), fmt, "the format is not set before the stream identifier");
    assert_eq!(slot(&corb, 6), sid, "the converter was never joined to the stream");
}
