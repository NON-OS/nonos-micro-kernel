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

//! The order of register writes inside one transfer. The databook makes
//! IC_TAR read-only while the core is enabled, and a target written at the
//! wrong moment is a transfer that quietly goes to the previous device.

use nonos_i2cmodel::regs::{IC_ENABLE, IC_TAR};
use nonos_i2cmodel::{Config, Violation};

use super::fixture::{bench, core, PAD};
use crate::transaction::{transfer, TransferRequest};

#[test]
fn the_target_is_written_only_while_the_core_is_disabled() {
    let b = bench(Config::LPSS);
    let req =
        TransferRequest { addr: PAD, flags: 0, write: &[0x05, 0x00, 0x00, 0x08], read_len: 0 };
    transfer(&b.driver, req).ok().expect("write transfer");
    core(|c| {
        let events = c.events();
        let mut enabled = 0;
        for e in events {
            if e.offset == IC_ENABLE {
                enabled = e.value & 1;
            }
            if e.offset == IC_TAR {
                assert_eq!(enabled, 0, "IC_TAR written with the core enabled");
            }
        }
        assert!(events.iter().any(|e| e.offset == IC_TAR), "the target was never written");
        let dropped: Vec<_> = c
            .violations()
            .iter()
            .filter(|v| matches!(v, Violation::WriteWhileEnabled { .. }))
            .collect();
        assert!(dropped.is_empty(), "writes the core dropped: {dropped:?}");
    });
}

#[test]
fn a_second_transfer_to_another_address_retargets_the_core() {
    let b = bench(Config::LPSS);
    let req =
        TransferRequest { addr: PAD, flags: 0, write: &[0x05, 0x00, 0x00, 0x08], read_len: 0 };
    transfer(&b.driver, req).ok().expect("write transfer");
    let elsewhere = TransferRequest { addr: 0x2C, flags: 0, write: &[0], read_len: 0 };
    assert!(transfer(&b.driver, elsewhere).is_err(), "nothing lives at 0x2C");
    assert_eq!(core(|c| c.writes_to(IC_TAR)), [PAD as u32, 0x2C]);
}
