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

//! Marks the capsule market unavailable for the rest of this boot. Discovery
//! already does this when the service is absent; a service that registers but
//! never answers is the same thing from the shell's side, so the startup
//! healthcheck routes here instead of failing the whole desktop.

use core::sync::atomic::Ordering;

pub fn disable_market() {
    super::constants::MARKET_DISABLED.store(true, Ordering::Relaxed);
}
