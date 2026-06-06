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
use core::sync::atomic::AtomicU64;

#[repr(C)]
pub struct Stats {
    pub uptime_requests: u64,
    pub bytes_served: u64,
    pub last_reseed_request: u64,
    pub source_failures: u64,
}

pub struct Pool {
    pub(in crate::pool) requests: AtomicU64,
    pub(in crate::pool) bytes_served: AtomicU64,
    pub(in crate::pool) last_reseed_request: AtomicU64,
    pub(in crate::pool) source_failures: AtomicU64,
}
