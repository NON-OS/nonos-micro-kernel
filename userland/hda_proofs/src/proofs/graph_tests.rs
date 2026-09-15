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

//! Walking a codec that answers, but answers nonsense.
//!
//! The node counts driving this walk come from the codec, so the codec decides
//! how long the walk runs. A part that reports 255 function groups, or that
//! returns the same word to every parameter read because its firmware never
//! loaded, would otherwise have the capsule question hundreds of nodes that do
//! not exist before giving up. Each question is a full ring round trip, so an
//! unbounded walk is a boot that appears to stop.

use nonos_devmodel::run;

use crate::constants::CORBWP;
use crate::controller::graph;
use crate::model::{corb_engine, rings, window};
use crate::regs::Regs;

/// A codec answering every parameter read with node start 2 and a count of
/// 255, none of which is an audio function group.
const CROWDED: u32 = 0x0002_00ff;
/// The walk cap in `controller/graph.rs`.
const WALK_CAP: u16 = 64;

#[test]
fn a_codec_reporting_no_function_groups_ends_the_walk_after_one_question() {
    let bar = window();
    let (corb, rirb) = rings(0);
    let _controller = run(&bar, corb_engine);
    let mut wp = 0u16;
    let regs = Regs::new(bar.base());
    assert!(graph::find_output(regs, corb.base(), rirb.base(), &mut wp, 0).is_none());
    assert_eq!(wp, 1, "an empty codec was asked more than the one question it answers");
}

#[test]
fn a_codec_claiming_more_nodes_than_exist_is_walked_no_further_than_the_cap() {
    let bar = window();
    let (corb, rirb) = rings(CROWDED);
    let _controller = run(&bar, corb_engine);
    let mut wp = 0u16;
    let regs = Regs::new(bar.base());
    assert!(graph::find_output(regs, corb.base(), rirb.base(), &mut wp, 0).is_none());
    assert_eq!(wp, WALK_CAP + 1, "the walk did not stop at the cap");
    assert_eq!(bar.wrote16(CORBWP as usize), WALK_CAP + 1);
}
