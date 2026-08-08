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

//! Hardware bring-up orchestrator. Reset -> station address -> RAL/RAH
//! + MTA -> RX ring -> TX ring. The Driver's existing RX/TX ring
//! state is programmed in place; the server loop reads them
//! through `&mut driver.rx` / `&mut driver.tx` after this returns.

use crate::setup::Driver;

use super::{mac_filter, reset, rx_setup, station_address, tx_setup};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    reset::run(&driver.regs)?;
    // Drawn, not read out of the EEPROM. The factory address identifies this
    // card to every network it ever joins, which outlives a system that keeps
    // nothing on disk.
    let mac = station_address::draw()?;
    driver.mac = mac;
    mac_filter::program(&driver.regs, &mac);
    rx_setup::program(&driver.regs, &driver.rx, driver.rx_ring_device_addr);
    tx_setup::program(&driver.regs, &driver.tx, driver.tx_ring_device_addr);
    Ok(())
}
