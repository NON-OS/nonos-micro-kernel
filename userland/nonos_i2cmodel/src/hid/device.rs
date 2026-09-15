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

//! The device's state. A transaction writes a register address and then
//! either keeps writing (a command) or turns round with a repeated START and
//! reads what that register holds. A STOP forgets the register: this is a
//! strict device, and the specification permits it.

use std::collections::{HashMap, VecDeque};

use super::descriptor::{self, Registers, LEN};
use super::record::Command;

pub struct HidOverI2c {
    pub(super) addr: u8,
    pub(super) regs: Registers,
    pub(super) descriptor: [u8; LEN],
    pub(super) report_desc: Vec<u8>,
    /// The register the current transaction addressed, once two bytes came.
    pub(super) pointer: Option<u16>,
    pub(super) written: Vec<u8>,
    /// What the current read phase is serving, and how far it got.
    pub(super) serving: Vec<u8>,
    pub(super) served: usize,
    pub(super) inputs: VecDeque<Vec<u8>>,
    pub(super) features: HashMap<u8, Vec<u8>>,
    pub(super) commands: Vec<Command>,
    pub(super) over_reads: u32,
    pub(super) acks_data: bool,
}

impl HidOverI2c {
    pub fn new(addr: u8, report_desc: Vec<u8>, max_input: u16, vendor: u16, product: u16) -> Self {
        let regs = descriptor::REGISTERS;
        let descriptor =
            descriptor::build(&regs, report_desc.len() as u16, max_input, vendor, product);
        Self {
            addr,
            regs,
            descriptor,
            report_desc,
            pointer: None,
            written: Vec::new(),
            serving: Vec::new(),
            served: 0,
            inputs: VecDeque::new(),
            features: HashMap::new(),
            commands: Vec::new(),
            over_reads: 0,
            acks_data: true,
        }
    }
}
