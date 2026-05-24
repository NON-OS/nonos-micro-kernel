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

use super::error::CfiError;
use super::stack::{ShadowStack, SHADOW_STACK_CAPACITY};
use spin::Mutex;

static SHADOW_STACK: Mutex<ShadowStack> = Mutex::new(ShadowStack::new());

pub fn enable_shadow_stack() {
    SHADOW_STACK.lock().enable();
}

pub fn shadow_stack_enabled() -> bool {
    SHADOW_STACK.lock().is_enabled()
}

pub const fn shadow_stack_capacity() -> usize {
    SHADOW_STACK_CAPACITY
}

pub fn push_shadow_return(ra: usize) -> Result<(), CfiError> {
    SHADOW_STACK.lock().push(ra)
}

pub fn pop_shadow_return() -> Result<usize, CfiError> {
    SHADOW_STACK.lock().pop()
}
