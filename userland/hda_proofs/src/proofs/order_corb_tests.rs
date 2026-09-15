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

//! What the command engine can see at the moment it is switched on.
//!
//! The specification is explicit that a ring's base and size are programmed
//! only while its DMA engine is stopped, and the reason is the same one that
//! makes this observable at all: the engine begins fetching on the edge that
//! sets RUN. Whatever the base registers hold at that instant is the memory it
//! reads, and a base still at zero is a bus master pointed at physical page
//! zero, which on this system is not the driver's to touch.

use std::sync::Arc;

use nonos_devmodel::{run, FakeBar};

use crate::constants::{CORBCTL, CORBCTL_RUN, CORBLBASE, CORBUBASE, RIRBLBASE, RIRBUBASE};
use crate::controller::corb;
use crate::model::WINDOW;
use crate::proofs::at_start::{wait_for, watch, AtStart};
use crate::proofs::corb_tests::{CORB_PA, RIRB_PA};
use crate::regs::Regs;

#[test]
fn both_rings_are_described_to_the_controller_before_either_engine_starts() {
    let bar = Arc::new(FakeBar::new(WINDOW));
    let s = Arc::new(AtStart::default());
    let at = [CORBLBASE, CORBUBASE, RIRBLBASE, RIRBUBASE].map(|r| r as usize);
    let gate = CORBCTL as usize;
    let device = run(&bar, watch(Arc::clone(&s), gate, CORBCTL_RUN, at));
    assert!(corb::init(Regs::new(bar.base()), CORB_PA, RIRB_PA).is_ok(), "ring bring-up");
    wait_for(&s);
    drop(device);
    assert_eq!(s.at(0), CORB_PA as u32, "the command engine started on an unset base");
    assert_eq!(s.at(1), (CORB_PA >> 32) as u32, "it started on half an address");
    assert_eq!(s.at(2), RIRB_PA as u32, "the response ring was still unset");
    assert_eq!(s.at(3), (RIRB_PA >> 32) as u32, "the response ring had half an address");
}
