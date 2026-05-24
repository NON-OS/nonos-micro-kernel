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

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use super::constants::DEFAULT_BASE;
use super::{Plic, PlicError, PlicResult};

static PLIC_BASE: AtomicU64 = AtomicU64::new(DEFAULT_BASE);
static PLIC_PRESENT: AtomicBool = AtomicBool::new(false);

pub fn plic_present() -> bool {
    PLIC_PRESENT.load(Ordering::Acquire)
}

pub fn install(base: u64) -> PlicResult<()> {
    if base == 0 {
        return Err(PlicError::InvalidBase);
    }
    PLIC_BASE.store(base, Ordering::Release);
    PLIC_PRESENT.store(true, Ordering::Release);
    Ok(())
}

pub fn current_plic() -> PlicResult<Plic> {
    if !plic_present() {
        return Err(PlicError::Absent);
    }
    Ok(Plic::new(PLIC_BASE.load(Ordering::Acquire)))
}
