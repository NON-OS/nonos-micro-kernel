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

//! Releasing the LPSS reset before anything else is touched.
//!
//! init/mod.rs:58-65 states it: "On Skylake and later the wrapper powers up
//! with the function and iDMA resets asserted, which leaves the core dead.
//! Writing LPSS_PRIV_RESETS = 0x7 deasserts them. This has to run once,
//! before the first DesignWare register access in disable()."
//!
//! Order is the property, not presence. A deassert that happens after the
//! core has already been configured leaves those writes to a block still in
//! reset, so they land nowhere and the controller comes up looking healthy
//! and carrying none of its settings.

use crate::constants::{
    IC_CON, IC_SS_SCL_HCNT, LPSS_PRIV, LPSS_PRIV_RESETS, LPSS_PRIV_RESETS_DEASSERT,
    LPSS_PRIV_RESETS_FUNC, LPSS_PRIV_RESETS_IDMA,
};
use crate::init::bring_up;
use crate::model::{live, refusal, stuck_at, CLOCK_HZ};
use crate::regs::Regs;

#[test]
fn the_reset_is_deasserted_before_any_designware_register_is_written() {
    /*
     * The dead-MMIO refusal is the earliest place bring-up can stop, and it
     * stops before IC_CON and the SCL counts are programmed. So a window that
     * fails the guard freezes the sequence at exactly the point of interest:
     * whatever is in the window is what ran ahead of the first DesignWare
     * write. The reset value has to be there and the core registers must not.
     */
    let bar = stuck_at(0x00);
    refusal(bring_up(Regs::new(bar.base()), CLOCK_HZ));

    assert_eq!(bar.wrote32(LPSS_PRIV_RESETS as usize), LPSS_PRIV_RESETS_DEASSERT);
    assert_eq!(bar.wrote32(IC_CON as usize), 0, "IC_CON written before the deassert");
    assert_eq!(bar.wrote32(IC_SS_SCL_HCNT as usize), 0, "SCL counts written too early");
}

#[test]
fn a_completed_bring_up_still_leaves_the_reset_deasserted() {
    /*
     * Nothing later in the sequence may re-assert it, whether by writing the
     * register again or by mapping another register onto the same offset.
     */
    let bar = live();
    bring_up(Regs::new(bar.base()), CLOCK_HZ).expect("bring-up");
    assert_eq!(bar.wrote32(LPSS_PRIV_RESETS as usize), LPSS_PRIV_RESETS_DEASSERT);
}

#[test]
fn the_deassert_value_releases_the_function_and_both_idma_resets() {
    /*
     * Deasserting only the function reset leaves the integrated DMA held,
     * which the DesignWare core needs out of reset before it will run.
     */
    assert_eq!(LPSS_PRIV_RESETS, LPSS_PRIV + 0x04);
    assert_eq!(LPSS_PRIV_RESETS_DEASSERT, 0x7);
    assert_eq!(LPSS_PRIV_RESETS_DEASSERT, LPSS_PRIV_RESETS_FUNC | LPSS_PRIV_RESETS_IDMA);
}
