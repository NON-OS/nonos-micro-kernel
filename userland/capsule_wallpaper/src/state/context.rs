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

use super::{FadeTimeline, Policy};

pub struct Context {
    pub compositor_port: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub backing_va: u64,
    pub argb: u32,
    pub alpha: u8,
    pub policy: Policy,
    pub fade: FadeTimeline,
    pub next_request_id: u32,
    pub policy_port: Option<u32>,
    pub catalog_port: Option<u32>,
    pub applied_wallpaper: Option<u8>,
    pub subscriber_ticks: u32,
}

impl Context {
    pub fn issue_request_id(&mut self) -> u32 {
        let id = self.next_request_id;
        self.next_request_id = id.wrapping_add(1).max(1);
        id
    }

    pub fn set_argb(&mut self, argb: u32) {
        self.argb = argb;
        self.alpha = (argb >> 24) as u8;
    }

    pub fn set_policy(&mut self, policy: Policy) {
        self.policy = policy;
    }

    pub fn current_argb(&self) -> u32 {
        let rgb = self.argb & 0x00FF_FFFF;
        ((self.alpha as u32) << 24) | rgb
    }
}
