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

use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

use super::buffer::RxBuffer;
use super::constants::{
    COM1_BASE, COM1_IRQ, COM2_BASE, COM2_IRQ, COM3_BASE, COM3_IRQ, COM4_BASE, COM4_IRQ,
    MAX_COM_PORTS,
};

#[derive(Debug)]
pub struct SerialStats {
    pub bytes_sent: AtomicU64,
    pub bytes_received: AtomicU64,
    pub rx_overruns: AtomicU64,
    pub parity_errors: AtomicU64,
    pub framing_errors: AtomicU64,
    pub break_interrupts: AtomicU64,
    pub fifo_errors: AtomicU64,
    pub interrupts: AtomicU64,
    pub tx_timeouts: AtomicU64,
}

impl SerialStats {
    pub const fn new() -> Self {
        Self {
            bytes_sent: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            rx_overruns: AtomicU64::new(0),
            parity_errors: AtomicU64::new(0),
            framing_errors: AtomicU64::new(0),
            break_interrupts: AtomicU64::new(0),
            fifo_errors: AtomicU64::new(0),
            interrupts: AtomicU64::new(0),
            tx_timeouts: AtomicU64::new(0),
        }
    }

    pub fn snapshot(&self) -> SerialStatsSnapshot {
        SerialStatsSnapshot {
            bytes_sent: self.bytes_sent.load(Ordering::Relaxed),
            bytes_received: self.bytes_received.load(Ordering::Relaxed),
            rx_overruns: self.rx_overruns.load(Ordering::Relaxed),
            parity_errors: self.parity_errors.load(Ordering::Relaxed),
            framing_errors: self.framing_errors.load(Ordering::Relaxed),
            break_interrupts: self.break_interrupts.load(Ordering::Relaxed),
            fifo_errors: self.fifo_errors.load(Ordering::Relaxed),
            interrupts: self.interrupts.load(Ordering::Relaxed),
            tx_timeouts: self.tx_timeouts.load(Ordering::Relaxed),
        }
    }

    pub fn reset(&self) {
        self.bytes_sent.store(0, Ordering::Relaxed);
        self.bytes_received.store(0, Ordering::Relaxed);
        self.rx_overruns.store(0, Ordering::Relaxed);
        self.parity_errors.store(0, Ordering::Relaxed);
        self.framing_errors.store(0, Ordering::Relaxed);
        self.break_interrupts.store(0, Ordering::Relaxed);
        self.fifo_errors.store(0, Ordering::Relaxed);
        self.interrupts.store(0, Ordering::Relaxed);
        self.tx_timeouts.store(0, Ordering::Relaxed);
    }
}

impl Default for SerialStats {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SerialStatsSnapshot {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub rx_overruns: u64,
    pub parity_errors: u64,
    pub framing_errors: u64,
    pub break_interrupts: u64,
    pub fifo_errors: u64,
    pub interrupts: u64,
    pub tx_timeouts: u64,
}

impl SerialStatsSnapshot {
    pub const fn total_errors(&self) -> u64 {
        self.rx_overruns + self.parity_errors + self.framing_errors + self.fifo_errors
    }
}

pub struct PortState {
    pub base: u16,
    pub irq: u8,
    pub index: u8,
    pub initialized: AtomicBool,
    pub rx_buffer: RxBuffer,
    pub stats: SerialStats,
}

impl PortState {
    pub const fn new(base: u16, irq: u8, index: u8) -> Self {
        Self {
            base,
            irq,
            index,
            initialized: AtomicBool::new(false),
            rx_buffer: RxBuffer::new(),
            stats: SerialStats::new(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Acquire)
    }

    pub fn set_initialized(&self, value: bool) {
        self.initialized.store(value, Ordering::Release);
    }
}

// SAFETY: Global state protected by atomic operations
static mut COM_PORTS: [PortState; MAX_COM_PORTS] = [
    PortState::new(COM1_BASE, COM1_IRQ, 0),
    PortState::new(COM2_BASE, COM2_IRQ, 1),
    PortState::new(COM3_BASE, COM3_IRQ, 2),
    PortState::new(COM4_BASE, COM4_IRQ, 3),
];

static PRIMARY_PORT: AtomicUsize = AtomicUsize::new(0);
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn get_port(index: usize) -> Option<&'static PortState> {
    if index >= MAX_COM_PORTS {
        return None;
    }
    // SAFETY: Index bounds checked, atomic operations protect concurrent access
    Some(unsafe { &COM_PORTS[index] })
}

pub fn get_port_mut(index: usize) -> Option<&'static mut PortState> {
    if index >= MAX_COM_PORTS {
        return None;
    }
    // SAFETY: Index bounds checked, caller must ensure exclusive access
    Some(unsafe { &mut COM_PORTS[index] })
}

pub fn primary_port_index() -> usize {
    PRIMARY_PORT.load(Ordering::Acquire)
}

pub fn set_primary_port(index: usize) {
    if index < MAX_COM_PORTS {
        PRIMARY_PORT.store(index, Ordering::Release);
    }
}

pub fn is_initialized() -> bool {
    INITIALIZED.load(Ordering::Acquire)
}

pub fn set_initialized(value: bool) -> bool {
    INITIALIZED.swap(value, Ordering::SeqCst)
}
