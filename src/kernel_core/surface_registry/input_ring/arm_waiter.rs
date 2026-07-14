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

#[cfg(feature = "input-probe-inject")]
use super::ring::INPUT_CONSUMER_READY;
use super::ring::WAITER;

pub fn arm_input_waiter(pid: u32) {
    WAITER.store(pid as u64, Ordering::Release);
    #[cfg(feature = "input-probe-inject")]
    INPUT_CONSUMER_READY.store(true, Ordering::Release);
}
