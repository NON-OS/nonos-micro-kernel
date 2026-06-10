// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use core::sync::atomic::{AtomicU64, Ordering};

const CANARY_MAGIC: u64 = 0xDEAD_BEEF_CAFE_BABE;
static STACK_CANARY: AtomicU64 = AtomicU64::new(0);
static HEAP_CANARY: AtomicU64 = AtomicU64::new(0);

pub fn init_canaries(entropy: u64) {
    STACK_CANARY.store(CANARY_MAGIC ^ entropy, Ordering::Release);
    HEAP_CANARY.store(CANARY_MAGIC ^ entropy.rotate_left(32), Ordering::Release);
}

pub fn get_stack_canary() -> u64 {
    STACK_CANARY.load(Ordering::Acquire)
}
pub fn get_heap_canary() -> u64 {
    HEAP_CANARY.load(Ordering::Acquire)
}

#[inline(never)]
fn constant_time_eq_u64(a: u64, b: u64) -> bool {
    (a ^ b) == 0
}

pub fn verify_stack_canary(expected: u64) -> bool {
    constant_time_eq_u64(STACK_CANARY.load(Ordering::Acquire), expected)
}
pub fn verify_heap_canary(expected: u64) -> bool {
    constant_time_eq_u64(HEAP_CANARY.load(Ordering::Acquire), expected)
}
