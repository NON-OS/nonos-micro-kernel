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

//! Software reset, against a part that completes it and one that does not.

use super::model::{live, resetting_part, window};
use crate::constants::regs::{CMD_RESET, REG_CMD};
use crate::init::reset_run;
use crate::regs::Regs;

#[test]
fn reset_is_requested_and_waited_for() {
    let bar = window();
    let _part = live(&bar, resetting_part);
    reset_run(&Regs::new(bar.base())).expect("a conforming part completes reset");
    assert_eq!(bar.wrote8(REG_CMD) & CMD_RESET, 0);
}

#[test]
fn a_part_that_never_completes_reset_is_given_up_on() {
    let bar = window();
    assert!(reset_run(&Regs::new(bar.base())).is_err());
    assert_ne!(bar.wrote8(REG_CMD) & CMD_RESET, 0, "the request was made");
}
