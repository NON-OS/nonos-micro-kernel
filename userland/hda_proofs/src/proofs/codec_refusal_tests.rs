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

//! Surviving a codec status register that is telling the truth about nothing.
//!
//! STATESTS is read once, early, and every bit in it becomes a codec the
//! capsule then tries to talk to. On a controller whose BAR decodes but whose
//! link never trained it reads back as all ones, which is fifteen codecs that
//! will never answer. Each one has to be refused on a bound rather than waited
//! on, and none of them may be reported as working.

use crate::constants::{IC, IRS, IRS_BUSY};
use crate::controller::probe;
use crate::model::window;
use crate::regs::Regs;

#[test]
fn a_status_register_reading_all_ones_yields_no_working_codec() {
    let bar = window();
    let found = probe(Regs::new(bar.base()), 0xffff);
    for (i, p) in found.iter().enumerate() {
        assert_eq!(p.present, 1, "codec {i} was claimed by the status register");
        assert_eq!(p.ok, 0, "codec {i} was reported working without ever answering");
        assert_eq!((p.vendor_id, p.device_id), (0, 0), "codec {i} invented an identity");
    }
}

#[test]
fn the_sixteenth_status_bit_is_not_treated_as_a_codec() {
    /*
     * The link carries fifteen addresses. Reading bit 15 as a codec walks one
     * past the array the capsule hands back, and on a register full of ones
     * that is exactly the bit an undecoded read supplies.
     */
    let bar = window();
    let found = probe(Regs::new(bar.base()), 1 << 15);
    for (i, p) in found.iter().enumerate() {
        assert_eq!(p.present, 0, "slot {i} answered to a bit with no codec behind it");
    }
}

#[test]
fn an_interface_left_busy_is_refused_before_a_command_is_written_over_it() {
    /*
     * The busy bit means a command is still in flight. Writing the next one
     * anyway replaces a verb another agent is waiting on, and the response
     * that eventually arrives is matched to the wrong request by both sides.
     */
    let bar = window();
    bar.present8(IRS as usize, IRS_BUSY);
    let found = probe(Regs::new(bar.base()), 0b1);
    assert_eq!(found[0].ok, 0, "a codec was believed over a busy interface");
    assert_eq!(bar.wrote32(IC as usize), 0, "a command was written over one still in flight");
}
