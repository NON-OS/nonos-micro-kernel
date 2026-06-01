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

use super::layer::{Layer, MAX_LAYERS};

pub struct SceneTable {
    pub(super) entries: [Layer; MAX_LAYERS],
    pub(super) count: usize,
}

impl SceneTable {
    pub const fn new() -> Self {
        Self {
            entries: [Layer {
                owner_pid: 0,
                surface_handle: 0,
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                z: 0,
                in_use: false,
                miss_count: 0,
            }; MAX_LAYERS],
            count: 0,
        }
    }
    pub fn submit(&mut self, layer: Layer) -> Result<(), ()> {
        for slot in self.entries.iter_mut() {
            if slot.in_use && slot.owner_pid == layer.owner_pid {
                *slot = layer;
                return Ok(());
            }
        }
        if self.count >= MAX_LAYERS {
            return Err(());
        }
        for slot in self.entries.iter_mut() {
            if !slot.in_use {
                *slot = layer;
                self.count += 1;
                return Ok(());
            }
        }
        Err(())
    }
    pub fn layers(&self) -> impl Iterator<Item = &Layer> {
        self.entries.iter().filter(|l| l.in_use)
    }
    pub fn drop_by_pid(&mut self, owner_pid: u32) -> u32 {
        let mut dropped = 0u32;
        for slot in self.entries.iter_mut() {
            if slot.in_use && slot.owner_pid == owner_pid {
                *slot = Layer::default();
                self.count = self.count.saturating_sub(1);
                dropped += 1;
            }
        }
        dropped
    }
    pub fn reap_unattachable(&mut self, attached: &[u64], threshold: u16, dropped: &mut [u64]) -> usize {
        let mut n = 0;
        for slot in self.entries.iter_mut() {
            if !slot.in_use {
                continue;
            }
            if attached.contains(&slot.surface_handle) {
                slot.miss_count = 0;
                continue;
            }
            slot.miss_count = slot.miss_count.saturating_add(1);
            if slot.miss_count >= threshold && n < dropped.len() {
                dropped[n] = slot.surface_handle;
                n += 1;
                *slot = Layer::default();
                self.count = self.count.saturating_sub(1);
            }
        }
        n
    }
}
