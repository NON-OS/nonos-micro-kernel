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

//! A register window laid out like an xHCI controller, and the devices that
//! answer from it.

use std::sync::Arc;

use nonos_devmodel::FakeBar;

use crate::constants::{
    CAPLENGTH, DBOFF, HCCPARAMS1, HCSPARAMS1, HCSPARAMS2, PORTSC_BASE, RTSOFF, USBSTS, USBSTS_CNR,
    USBSTS_HCH,
};

pub const CAP_LEN: usize = 0x20;
pub const RUNTIME: usize = 0x1000;
pub const DOORBELL: usize = 0x1800;
/// In 32-bit words from the cap base, as HCCPARAMS1 carries it.
pub const XECP_WORDS: u32 = 0x200;
pub const LEGACY: usize = XECP_WORDS as usize * 4;
pub const PORT1: usize = CAP_LEN + PORTSC_BASE as usize;

/// A controller with 8 slots, 2 ports, 4 scratchpads, 64-bit addressing and a
/// legacy capability at `LEGACY`. `xecp` zero leaves the capability out.
pub fn controller(xecp: u32) -> Arc<FakeBar> {
    let bar = Arc::new(FakeBar::new(0x2000));
    bar.present32(CAPLENGTH as usize, CAP_LEN as u32);
    bar.present32(HCSPARAMS1 as usize, (2 << 24) | 8);
    bar.present32(HCSPARAMS2 as usize, 4 << 27);
    bar.present32(HCCPARAMS1 as usize, (xecp << 16) | 0x1);
    bar.present32(DBOFF as usize, DOORBELL as u32);
    bar.present32(RTSOFF as usize, RUNTIME as u32);
    bar.present32(CAP_LEN + USBSTS as usize, USBSTS_CNR | USBSTS_HCH);
    bar
}

pub fn op_base(bar: &FakeBar) -> u64 {
    bar.base() + CAP_LEN as u64
}

/// Two halves of a 64-bit register the driver wrote.
pub fn wrote64(bar: &FakeBar, off: usize) -> u64 {
    bar.wrote32(off) as u64 | (bar.wrote32(off + 4) as u64) << 32
}

pub const BIOS_OWNED: u32 = 1 << 16;
pub const OS_OWNED: u32 = 1 << 24;

/// The controller with a legacy capability, firmware holding it or not.
pub fn with_legacy_cap(bios_owned: bool) -> Arc<FakeBar> {
    let bar = controller(XECP_WORDS);
    bar.present32(LEGACY, 1 | if bios_owned { BIOS_OWNED } else { 0 });
    bar.present32(LEGACY + 4, 0xFFFF_FFFF);
    bar
}
