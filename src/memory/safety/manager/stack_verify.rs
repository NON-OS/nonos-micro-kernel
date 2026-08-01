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

use super::super::constants::CANARY_BASE;
use super::super::types::GuardType;
use super::guards::get_guard_regions;
use super::helpers::is_guard_compromised;
use crate::memory::layout;

pub fn verify_stack_integrity() -> bool {
    let guards = get_guard_regions();
    for guard in guards {
        if matches!(guard.region_type, GuardType::StackGuard) {
            if is_guard_compromised(guard.start, guard.end - guard.start) {
                return false;
            }
        }
    }

    let current_rsp = crate::arch::stack_pointer();

    for region in layout::get_all_stack_regions() {
        if current_rsp >= region.base && current_rsp < region.base + region.size as u64 {
            let canary_addr = region.base + region.size as u64 - 8;
            unsafe {
                let canary = (canary_addr as *const u64).read_volatile();
                if canary != CANARY_BASE {
                    return false;
                }
            }
        }
    }
    true
}
