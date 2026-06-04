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

use super::types::{Pool, Stats};

impl Pool {
    pub fn snapshot(&self) -> Stats {
        Stats {
            uptime_requests: self.requests.load(Ordering::Relaxed),
            bytes_served: self.bytes_served.load(Ordering::Relaxed),
            last_reseed_request: self.last_reseed_request.load(Ordering::Relaxed),
            source_failures: self.source_failures.load(Ordering::Relaxed),
        }
    }
}
