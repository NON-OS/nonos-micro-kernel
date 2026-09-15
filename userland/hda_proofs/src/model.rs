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

//! The controllers the tests put in front of the driver.
//!
//! A window on its own is a dead controller: it reads back what was written
//! and advances nothing. That is a real state, the one a driver has to survive
//! without hanging, so it is used deliberately. The two models here are the
//! live halves, each written from what the HD Audio specification says a
//! conforming controller does with a register the driver never writes.

use std::sync::Arc;

use nonos_devmodel::FakeBar;

use crate::constants::{CORBWP, IRS, IRS_BUSY, IRS_VALID, RIRBWP};
use crate::controller::StreamDescriptor;

/// The smallest HDA BAR the capsule accepts. It covers the global registers,
/// both ring control blocks and the stream descriptors.
pub const WINDOW: usize = 0x1000;
/// A 256-entry CORB and RIRB, the sizes `corb::init` programs.
pub const CORB_BYTES: usize = 256 * 4;
pub const RIRB_BYTES: usize = 256 * 8;

pub fn window() -> Arc<FakeBar> {
    Arc::new(FakeBar::new(WINDOW))
}

/// Command and response rings in host memory, with `response` already in every
/// RIRB slot. Pre-filling leaves the live controller below one store to make,
/// so the driver cannot see an advanced write pointer over a stale response.
pub fn rings(response: u32) -> (Arc<FakeBar>, Arc<FakeBar>) {
    let rirb = FakeBar::new(RIRB_BYTES);
    for slot in 0..256 {
        rirb.present32(slot * 8, response);
    }
    (Arc::new(FakeBar::new(CORB_BYTES)), Arc::new(rirb))
}

/// A controller whose CORB engine is running: it consumes every command the
/// driver posts and advances RIRBWP to match. RIRBWP belongs to the controller
/// and the driver only reads it, so nothing here races.
pub fn corb_engine(bar: &FakeBar) {
    bar.present16(RIRBWP as usize, bar.wrote16(CORBWP as usize) & 0xff);
}

/// A codec answering on the immediate command interface: the driver raises
/// IRS_BUSY, the controller drops it and raises IRS_VALID.
pub fn immediate_responder(bar: &FakeBar) {
    if bar.wrote8(IRS as usize) & IRS_BUSY != 0 {
        bar.present8(IRS as usize, IRS_VALID);
    }
}

/// The descriptor the layout gives global stream `index`.
pub fn descriptor(index: u16) -> StreamDescriptor {
    let mmio_offset = 0x80 + index as u32 * 0x20;
    StreamDescriptor { kind: 2, local_index: 0, global_index: index, mmio_offset }
}
