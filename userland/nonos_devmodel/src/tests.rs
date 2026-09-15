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

//! The harness proving it can do the thing it exists for.
//!
//! These do not test a driver. They stand in for one, running the shape of
//! access a real bring-up performs so the harness itself is known to work
//! before a driver's correctness is argued from it. Without this, a driver test
//! that passed against a live model would leave open whether the model ever
//! actually answered.
//!
//! The shape below is the two-edge reset handshake that appears in the HDA
//! controller's CORB bring-up and in xHCI's port reset: set a bit, wait for the
//! device to echo it, clear it, wait for the device to drop it. It is the case
//! a passive window cannot serve at all, because whichever value it is
//! preloaded with, one of the two waits never finishes.

use std::sync::Arc;

use super::{run, FakeBar};

const CTL: usize = 0x00;
const STATUS: usize = 0x04;
const RESET: u32 = 1 << 0;
const SPINS: u32 = 5_000_000;

/// A controller that mirrors the reset bit from its control register into its
/// status register, which is what the specifications above require of one.
fn mirroring_device(bar: &FakeBar) {
    let asserted = bar.wrote32(CTL) & RESET != 0;
    bar.present32(STATUS, if asserted { RESET } else { 0 });
}

/// The driver half: assert, wait for the echo, deassert, wait for the drop.
fn reset_sequence(bar: &FakeBar) -> Result<(), &'static str> {
    bar.present32(CTL, RESET);
    let mut spins = 0;
    while bar.wrote32(STATUS) & RESET == 0 {
        spins += 1;
        if spins > SPINS {
            return Err("device never echoed the reset");
        }
    }
    bar.present32(CTL, 0);
    spins = 0;
    while bar.wrote32(STATUS) & RESET != 0 {
        spins += 1;
        if spins > SPINS {
            return Err("device never dropped the reset");
        }
    }
    Ok(())
}

#[test]
fn a_live_device_answers_both_edges_of_a_reset_handshake() {
    let bar = Arc::new(FakeBar::new(64));
    let _device = run(&bar, mirroring_device);
    reset_sequence(&bar).expect("a mirroring device must complete the handshake");
}

#[test]
fn a_passive_window_cannot_answer_the_same_handshake() {
    /*
     * The control. No device, so the status register never changes and the
     * first wait runs out. This is what the whole module exists to fix, and
     * asserting it keeps the previous test from passing for a trivial reason.
     */
    let bar = Arc::new(FakeBar::new(64));
    let err = reset_sequence(&bar).expect_err("a passive window must stall");
    assert!(err.contains("echoed"), "unexpected error: {err}");
}

#[test]
fn a_stopped_device_stops_answering() {
    /*
     * Dropping the guard joins the thread, so a test cannot leave a model
     * running against a window a later test reuses.
     */
    let bar = Arc::new(FakeBar::new(64));
    {
        let _device = run(&bar, mirroring_device);
        reset_sequence(&bar).expect("handshake while the device is live");
    }
    bar.present32(CTL, RESET);
    assert_eq!(bar.wrote32(STATUS), 0, "a dropped device must not still be reacting");
}
