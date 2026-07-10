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

// One slot per app that wants input. A full desktop with several base apps
// plus on-demand window instances (each a distinct pid) can exceed a couple
// dozen, and a subscriber that cannot get a slot receives no keyboard or
// pointer at all, so the ceiling is generous.
pub const MAX_SUBSCRIBERS: usize = 64;

#[derive(Clone, Copy, Default)]
pub struct Subscription {
    pub pid: u32,
    pub kind_mask: u32,
    pub in_use: bool,
}

pub struct SubscriptionTable {
    pub(crate) entries: [Subscription; MAX_SUBSCRIBERS],
}
