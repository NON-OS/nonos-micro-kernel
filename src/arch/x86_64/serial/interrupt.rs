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

use core::sync::atomic::Ordering;

use super::constants::{
    IIR_ID_MASK, IIR_NO_INT, LSR_BREAK_INT, LSR_DATA_READY, LSR_FIFO_ERR, LSR_FRAMING_ERR,
    LSR_OVERRUN_ERR, LSR_PARITY_ERR, MAX_COM_PORTS, REG_DATA, REG_IIR_FCR, REG_LSR, REG_MSR,
};
use super::ops::{is_data_ready, read_reg};
use super::state::get_port_mut;

pub fn handle_interrupt(port_index: usize) {
    if port_index >= MAX_COM_PORTS {
        return;
    }

    let state = match get_port_mut(port_index) {
        Some(s) => s,
        None => return,
    };

    if !state.is_initialized() {
        return;
    }

    state.stats.interrupts.fetch_add(1, Ordering::Relaxed);
    let base = state.base;

    loop {
        let iir = read_reg(base, REG_IIR_FCR);

        if iir & IIR_NO_INT != 0 {
            break;
        }

        match iir & IIR_ID_MASK {
            0x04 | 0x0C => {
                while is_data_ready(base) {
                    let byte = read_reg(base, REG_DATA);
                    state.stats.bytes_received.fetch_add(1, Ordering::Relaxed);

                    if !state.rx_buffer.push(byte) {
                        state.stats.rx_overruns.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
            0x02 => {
                // Transmitter empty
            }
            0x06 => {
                let lsr = read_reg(base, REG_LSR);

                if lsr & LSR_OVERRUN_ERR != 0 {
                    state.stats.rx_overruns.fetch_add(1, Ordering::Relaxed);
                }
                if lsr & LSR_PARITY_ERR != 0 {
                    state.stats.parity_errors.fetch_add(1, Ordering::Relaxed);
                }
                if lsr & LSR_FRAMING_ERR != 0 {
                    state.stats.framing_errors.fetch_add(1, Ordering::Relaxed);
                }
                if lsr & LSR_BREAK_INT != 0 {
                    state.stats.break_interrupts.fetch_add(1, Ordering::Relaxed);
                }
                if lsr & LSR_FIFO_ERR != 0 {
                    state.stats.fifo_errors.fetch_add(1, Ordering::Relaxed);
                }

                if lsr & LSR_DATA_READY != 0 {
                    let _ = read_reg(base, REG_DATA);
                }
            }
            0x00 => {
                let _ = read_reg(base, REG_MSR);
            }
            _ => {
                break;
            }
        }
    }
}

pub fn handle_com1_interrupt() {
    handle_interrupt(0);
}

pub fn handle_com2_interrupt() {
    handle_interrupt(1);
}

pub fn handle_com3_interrupt() {
    handle_interrupt(2);
}

pub fn handle_com4_interrupt() {
    handle_interrupt(3);
}
