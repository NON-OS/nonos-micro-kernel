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

//! Cold-start stage one: power the MAC on and confirm it answers. This needs no
//! DMA, so it is the first thing a boot on real silicon can report. The
//! firmware download (proven in `rtl8821ce_proofs`) builds on this once the TX
//! ring exists.

use crate::constants::regs::{mmio_dead, REG_SYS_CFG1};
use crate::pwr::{run_pwr_seq, CARD_ENABLE};
use crate::regs::Mmio;

/// How far cold-start got, reported on screen so a first boot is legible.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BringUp {
    /// The register window read back all-ones or all-zero after power-on: the
    /// BAR is not decoding or the card has no power.
    DeadMmio,
    /// The power-on sequence never reached readiness.
    PowerFailed,
}

/// Power the MAC on and read a register back. Returns the chip-id readback, or
/// the stage that failed.
pub fn probe<M: Mmio>(mmio: &M) -> Result<u32, BringUp> {
    if !run_pwr_seq(mmio, CARD_ENABLE) {
        return Err(BringUp::PowerFailed);
    }
    let id = mmio.read32(REG_SYS_CFG1);
    if mmio_dead(id) {
        return Err(BringUp::DeadMmio);
    }
    Ok(id)
}
