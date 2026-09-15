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

//! Nothing answered, or the device answered its address and then refused
//! the data. Both are a NACK to the caller, and both have to leave the core
//! ready for the next transfer.

use nonos_i2cmodel::regs::{IC_RAW_INTR_STAT, INTR_TX_ABRT};
use nonos_i2cmodel::Config;

use super::fixture::{bench, bench_with, core, pad, PAD};
use crate::transaction::{transfer, TransferError, TransferRequest, TransferResult};

const NOBODY: u8 = 0x2C;

fn nack(outcome: Result<TransferResult, TransferError>) {
    match outcome {
        Err(TransferError::Nack) => {}
        Err(_) => panic!("refused, but not as a NACK"),
        Ok(_) => panic!("the transfer succeeded where the bus refused it"),
    }
}

#[test]
fn an_address_nobody_answers_is_a_nack_and_the_abort_is_acknowledged() {
    let b = bench(Config::LPSS);
    nack(transfer(&b.driver, TransferRequest { addr: NOBODY, flags: 0, write: &[0], read_len: 0 }));
    core(|c| {
        assert_eq!(c.read32(IC_RAW_INTR_STAT) & INTR_TX_ABRT, 0, "abort left latched");
        assert!(!c.is_enabled(), "controller left enabled after the abort");
    });
}

#[test]
fn a_device_that_refuses_its_data_is_a_nack_too() {
    let b = bench_with(Config::LPSS, pad().deaf());
    let req = TransferRequest { addr: PAD, flags: 0, write: &[0x05, 0x00], read_len: 0 };
    nack(transfer(&b.driver, req));
    assert!(core(|c| c.bus().trace().iter().any(|e| e.is_start())), "the address never went out");
}

#[test]
fn the_transfer_after_a_nack_goes_through() {
    let b = bench(Config::LPSS);
    nack(transfer(&b.driver, TransferRequest { addr: NOBODY, flags: 0, write: &[0], read_len: 0 }));
    let req = TransferRequest { addr: PAD, flags: 1, write: &[0x01, 0x00], read_len: 2 };
    let r = transfer(&b.driver, req).ok().expect("the bus is usable after a NACK");
    assert_eq!(r.read_len, 2);
}
