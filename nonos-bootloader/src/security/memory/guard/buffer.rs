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

const STACK_GUARD_PATTERN: u64 = 0x5555_AAAA_5555_AAAA;

pub struct GuardedBuffer<const N: usize> {
    guard_front: u64,
    data: [u8; N],
    guard_back: u64,
}

impl<const N: usize> GuardedBuffer<N> {
    pub fn new() -> Self {
        Self { guard_front: STACK_GUARD_PATTERN, data: [0u8; N], guard_back: STACK_GUARD_PATTERN }
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
    pub fn verify_guards(&self) -> bool {
        self.guard_front == STACK_GUARD_PATTERN && self.guard_back == STACK_GUARD_PATTERN
    }

    pub fn check_and_panic(&self) {
        if !self.verify_guards() {
            crate::log::logger::log_critical(
                "security",
                "SECURITY: Buffer overflow detected - secure halt",
            );
            loop {
                core::hint::spin_loop();
            }
        }
    }
}
