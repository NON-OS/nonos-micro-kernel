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

//! The core's state between accesses.

use std::collections::HashMap;

use super::config::Config;
use super::fifo::Fifo;
use super::record::{RegEvent, Violation};
use crate::bus::{Bus, Dir};

pub struct Designware {
    pub(super) config: Config,
    pub(super) bus: Bus,
    /// LPSS_PRIV_RESETS as last written. Zero out of power-on: the core is
    /// held in reset until the driver releases it.
    pub(super) resets: u32,
    pub(super) enable: u32,
    /// Whether IC_ENABLE_STATUS follows IC_ENABLE. A part whose functional
    /// clock is gated never moves it, and a driver has to survive that.
    pub(super) enable_follows: bool,
    /// Configuration registers, kept as written while the core was disabled.
    pub(super) regs: HashMap<u64, u32>,
    pub(super) tx: Fifo<u32>,
    pub(super) rx: Fifo<u8>,
    /// A transaction is open on the bus: a START went out and no STOP yet.
    pub(super) active: bool,
    pub(super) dir: Option<Dir>,
    pub(super) abort_source: u32,
    pub(super) raw_intr: u32,
    pub(super) violations: Vec<Violation>,
    pub(super) events: Vec<RegEvent>,
}

impl Designware {
    pub fn new(config: Config, bus: Bus) -> Self {
        Self {
            config,
            bus,
            resets: 0,
            enable: 0,
            enable_follows: true,
            regs: HashMap::new(),
            tx: Fifo::new(config.tx_depth),
            rx: Fifo::new(config.rx_depth),
            active: false,
            dir: None,
            abort_source: 0,
            raw_intr: 0,
            violations: Vec::new(),
            events: Vec::new(),
        }
    }
}
