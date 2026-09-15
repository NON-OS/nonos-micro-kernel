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

//! Giving up on a controller that never answers.
//!
//! This is the case the capsule meets on hardware it does not fully support:
//! the BAR decodes, the registers accept writes, and the DMA engine does
//! nothing. There is no interrupt and no error bit to read, so the only thing
//! separating a bounded refusal from a hung boot is that every wait counts its
//! spins. A driver that waits forever here takes the whole service down, and
//! with it anything that was waiting on the audio endpoint to come up.

use crate::controller::{graph, verb};
use crate::error::HdaError;
use crate::model::{rings, window};
use crate::proofs::verb_tests::{CMD, RESPONSE};
use crate::regs::Regs;

#[test]
fn a_controller_that_never_answers_is_given_up_on_rather_than_waited_on() {
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let mut wp = 0u16;
    let got = verb::send(Regs::new(bar.base()), corb.base(), rirb.base(), &mut wp, CMD);
    assert!(got == Err(HdaError::VerbTimeout), "an unanswered verb must refuse, not spin");
    assert_eq!(wp, 0, "the shadow pointer must not advance past a command nobody consumed");
}

#[test]
fn the_codec_graph_walk_ends_when_the_first_question_goes_unanswered() {
    /*
     * The walk is a chain of verbs, each feeding the next. If the refusal is
     * not propagated the walk carries on against invented node numbers and
     * eventually configures a path made entirely of timeouts.
     */
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let mut wp = 0u16;
    let regs = Regs::new(bar.base());
    let found = graph::find_output(regs, corb.base(), rirb.base(), &mut wp, 0);
    assert!(found.is_none(), "a silent codec must not yield an output path");
}
