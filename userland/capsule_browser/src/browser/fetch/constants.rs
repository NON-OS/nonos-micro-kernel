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

pub(super) const MAX_BODY: usize = 4 * 1024 * 1024;
pub(super) const MAX_TLS_FLIGHT: usize = 512 * 1024;
pub(super) const FIRST_WAIT: u32 = 25;
pub(super) const IDLE_AFTER: u32 = 20;
pub(super) const MAX_FETCH_MS: i64 = 12000;
pub(super) const MAX_REDIRECTS: u8 = 5;
pub(super) const DRAIN_BURST: usize = 64;
pub(super) const HS_WAIT: u32 = 200;
pub(super) const MAX_RETRIES: u8 = 2;
pub(super) const FLIGHT_SETTLE: u32 = 15;
