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

extern crate alloc;

use alloc::string::String;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use super::bus::IPC_BUS;
use super::hash::compute_channel_key;
use super::message::IpcMessage;

/// Internal channel registry entry
pub(super) struct ChannelEntry {
    /// Source module
    pub from: String,
    /// Destination module
    pub to: String,
    /// Channel key (hash of from+to)
    pub key: u64,
    /// Whether channel is alive
    pub alive: AtomicBool,
    /// Last activity timestamp
    pub last_active_ms: AtomicU64,
    /// Messages sent through this channel
    pub messages_sent: AtomicU64,
}

impl ChannelEntry {
    pub(super) fn new(from: &str, to: &str) -> Result<Self, &'static str> {
        Ok(Self {
            from: String::from(from),
            to: String::from(to),
            key: compute_channel_key(from, to)?,
            alive: AtomicBool::new(true),
            last_active_ms: AtomicU64::new(crate::time::timestamp_millis()),
            messages_sent: AtomicU64::new(0),
        })
    }

    pub(super) fn touch(&self) {
        self.last_active_ms.store(crate::time::timestamp_millis(), Ordering::Relaxed);
        self.alive.store(true, Ordering::Relaxed);
    }

    pub(super) fn record_send(&self) {
        self.messages_sent.fetch_add(1, Ordering::Relaxed);
        self.touch();
    }
}

#[derive(Clone, Copy)]
pub struct IpcChannel {
    pub(super) key: u64,
}

impl IpcChannel {
    /// Send a message on this channel
    #[inline]
    pub fn send(&self, msg: IpcMessage) -> Result<(), &'static str> {
        IPC_BUS.enqueue(self.key, msg)
    }

    /// Get the channel key
    #[inline]
    pub fn key(&self) -> u64 {
        self.key
    }
}

impl core::fmt::Debug for IpcChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "IpcChannel(key=0x{:016x})", self.key)
    }
}

