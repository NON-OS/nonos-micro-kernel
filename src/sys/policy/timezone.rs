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

use core::sync::atomic::{AtomicI8, Ordering};

const MIN_OFFSET: i8 = -12;
const MAX_OFFSET: i8 = 14;

static TIMEZONE_OFFSET: AtomicI8 = AtomicI8::new(0);

#[inline]
pub fn timezone_offset() -> i8 {
    TIMEZONE_OFFSET.load(Ordering::Relaxed)
}

pub(super) fn set_timezone_offset(value: i8) {
    let clamped = value.max(MIN_OFFSET).min(MAX_OFFSET);
    TIMEZONE_OFFSET.store(clamped, Ordering::Release);
}
