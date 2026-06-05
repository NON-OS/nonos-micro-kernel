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

use alloc::{collections::VecDeque, string::String, vec::Vec};
use core::sync::atomic::Ordering;
use spin::{Mutex, RwLock};

use super::channel::{ChannelEntry, IpcChannel};
use super::hash::compute_channel_key;
use super::message::IpcMessage;
use super::stats::{BusStats, BusStatsSnapshot};

pub const DEFAULT_MAX_QUEUE: usize = 4096;

pub const DEFAULT_MSG_TIMEOUT_MS: u64 = 5_000;

pub struct IpcBus {
    channels: RwLock<Vec<ChannelEntry>>,
    queue: Mutex<VecDeque<(u64, IpcMessage)>>,
    max_queue: usize,
    msg_timeout_ms: u64,
    stats: BusStats,
}

impl IpcBus {
    pub const fn new() -> Self {
        Self {
            channels: RwLock::new(Vec::new()),
            queue: Mutex::new(VecDeque::new()),
            max_queue: DEFAULT_MAX_QUEUE,
            msg_timeout_ms: DEFAULT_MSG_TIMEOUT_MS,
            stats: BusStats::new(),
        }
    }

    pub fn open_channel(
        &self,
        from: &str,
        to: &str,
        _token: &crate::syscall::capabilities::CapabilityToken,
    ) -> Result<(), &'static str> {
        if from.is_empty() || to.is_empty() {
            return Err("Invalid channel endpoints");
        }

        let mut ch = self.channels.write();

        if ch.iter().any(|c| c.from == from && c.to == to) {
            return Ok(());
        }

        ch.push(ChannelEntry::new(from, to)?);
        self.stats.channels_opened.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }

    pub fn find_channel(&self, from: impl AsRef<str>, to: impl AsRef<str>) -> Option<IpcChannel> {
        let f = from.as_ref();
        let t = to.as_ref();
        let Ok(key) = compute_channel_key(f, t) else { return None };

        let ch = self.channels.read();
        if ch.iter().any(|c| c.key == key && c.alive.load(Ordering::Relaxed)) {
            Some(IpcChannel { key })
        } else {
            None
        }
    }

    pub fn channel_exists(&self, from: &str, to: &str) -> bool {
        let Ok(key) = compute_channel_key(from, to) else { return false };
        let ch = self.channels.read();
        ch.iter().any(|c| c.key == key)
    }

    pub fn enqueue(&self, key: u64, msg: IpcMessage) -> Result<(), &'static str> {
        if let Some(c) = self.channels.read().iter().find(|c| c.key == key) {
            c.record_send();
        }

        let bytes = msg.data.len() as u64;

        let mut q = self.queue.lock();
        if q.len() >= self.max_queue {
            self.stats.queue_full_rejections.fetch_add(1, Ordering::Relaxed);
            return Err("IPC queue full");
        }

        q.push_back((key, msg));
        drop(q);

        self.stats.messages_enqueued.fetch_add(1, Ordering::Relaxed);
        self.stats.bytes_transferred.fetch_add(bytes, Ordering::Relaxed);

        Ok(())
    }

    pub fn get_next_message(&self) -> Option<IpcMessage> {
        let mut q = self.queue.lock();
        let result = q.pop_front().map(|(_key, msg)| msg);

        if result.is_some() {
            self.stats.messages_dequeued.fetch_add(1, Ordering::Relaxed);
        }

        result
    }

    pub fn get_timed_out_messages(&self) -> Vec<IpcMessage> {
        let now = crate::time::timestamp_millis();
        let mut q = self.queue.lock();
        let mut out = Vec::new();
        let mut remain = VecDeque::with_capacity(q.len());

        while let Some((key, msg)) = q.pop_front() {
            if now.saturating_sub(msg.timestamp_ms) > self.msg_timeout_ms {
                out.push(msg);
                self.stats.messages_timed_out.fetch_add(1, Ordering::Relaxed);

                if let Some(c) = self.channels.read().iter().find(|c| c.key == key) {
                    c.alive.store(false, Ordering::Relaxed);
                }
            } else {
                remain.push_back((key, msg));
            }
        }

        *q = remain;
        out
    }

    pub fn find_dead_channels(&self) -> Vec<usize> {
        let ch = self.channels.read();
        ch.iter()
            .enumerate()
            .filter(|(_i, c)| !c.alive.load(Ordering::Relaxed))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn remove_channel(&self, index: usize) {
        let mut ch = self.channels.write();
        if index < ch.len() {
            ch.remove(index);
            self.stats.channels_closed.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn remove_all_channels_for_module(&self, module: &str) {
        let mut ch = self.channels.write();
        let before = ch.len();
        ch.retain(|c| c.from != module && c.to != module);
        let removed = before - ch.len();
        self.stats.channels_closed.fetch_add(removed as u64, Ordering::Relaxed);
    }

    pub fn list_routes(&self) -> Vec<(String, String)> {
        self.channels.read().iter().map(|c| (c.from.clone(), c.to.clone())).collect()
    }

    pub fn get_active_channel_count(&self) -> usize {
        self.channels.read().len()
    }

    pub fn get_queue_depth(&self) -> usize {
        self.queue.lock().len()
    }

    pub fn send_system_message(
        &self,
        env: crate::ipc::nonos_message::IpcEnvelope,
    ) -> Result<(), &'static str> {
        let key = compute_channel_key(&env.from, &env.to)?;
        let msg = IpcMessage::new(&env.from, &env.to, &env.data)?;
        self.enqueue(key, msg)
    }

    pub fn get_stats(&self) -> BusStatsSnapshot {
        BusStatsSnapshot {
            messages_enqueued: self.stats.messages_enqueued.load(Ordering::Relaxed),
            messages_dequeued: self.stats.messages_dequeued.load(Ordering::Relaxed),
            messages_timed_out: self.stats.messages_timed_out.load(Ordering::Relaxed),
            channels_opened: self.stats.channels_opened.load(Ordering::Relaxed),
            channels_closed: self.stats.channels_closed.load(Ordering::Relaxed),
            bytes_transferred: self.stats.bytes_transferred.load(Ordering::Relaxed),
            queue_full_rejections: self.stats.queue_full_rejections.load(Ordering::Relaxed),
            current_queue_depth: self.get_queue_depth(),
            current_channel_count: self.get_active_channel_count(),
        }
    }
}

pub static IPC_BUS: IpcBus = IpcBus::new();
