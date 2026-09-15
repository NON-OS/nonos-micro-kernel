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

//! The whole sequence, with a part watching for the one order that matters.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::entropy;

use super::memory::Memory;
use super::model::{idr, live, resetting_part, window, FACTORY};
use crate::constants::regs::{CMD_RX_ENABLE, ISR_ENABLED, REG_CMD, REG_IMR};
use crate::init::bring_up;

/*
 * A part that completes resets and also watches: if it ever sees receive
 * enabled while the IDR bytes still hold the factory address, the driver went
 * on the air under the identifier it exists to avoid. The check is a state
 * predicate, so any observation after the enable write is enough to catch it.
 */
fn watching_part(leaked: Arc<AtomicBool>) -> impl Fn(&FakeBar) {
    move |bar| {
        resetting_part(bar);
        let cmd = bar.wrote8(REG_CMD);
        if cmd & CMD_RX_ENABLE != 0 && idr(bar) == FACTORY {
            leaked.store(true, Ordering::SeqCst);
        }
    }
}

#[test]
fn the_part_is_never_enabled_while_it_still_carries_the_factory_address() {
    let _turn = entropy(true);
    let bar = window();
    let leaked = Arc::new(AtomicBool::new(false));
    let _part = live(&bar, watching_part(leaked.clone()));
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    bring_up(&mut d).expect("bring-up completes");
    assert!(!leaked.load(Ordering::SeqCst), "receive was enabled with the factory address");
    assert_eq!(d.mac, idr(&bar));
    let cmd = bar.wrote8(REG_CMD);
    assert_eq!(cmd & 0x1C, 0x0C, "TE bit 2 and RE bit 3 on, RST bit 4 off, per the datasheet");
    assert_eq!(bar.wrote16(REG_IMR), ISR_ENABLED);
}
