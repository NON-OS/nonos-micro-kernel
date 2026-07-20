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

//! Baseband and RF bring-up for the RTL8821CE: power the BB/RF domains on, then
//! load the vendor baseband, AGC, RF and PHY-MAC register tables through the
//! condition parser. Channel selection and the on-chip RF calibration (which the
//! firmware runs on the 8051) build on this. The parser, the table application
//! widths and the RF SIPI encoding are proven off-silicon; the chip condition
//! (cut version and RF front-end option) is read from the running card.

pub mod apply;
// IQK is transmit-side calibration and rides an H2C packet down the H2C queue,
// which the receive-capable bring-up does not set up; it is exercised in the
// proofs and wired when the H2C packet path lands. Kept out of the capsule build
// so the receive path stays warning-clean.
#[cfg(test)]
pub mod calib;
pub mod channel;
pub mod cond;
pub mod power;
pub mod regs;
pub mod rf;
pub mod rxpath;
mod tables;
pub mod txpower;

pub use apply::load_all;
pub use cond::{PhyCond, INTF_PCIE};
pub use power::power_on;
