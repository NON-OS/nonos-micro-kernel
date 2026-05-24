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

mod error;
mod mode;
mod stack;
mod state;

pub use error::CfiError;
pub use mode::CfiMode;

pub fn init_cfi() {
    state::enable_shadow_stack();
}

pub fn cfi_supported() -> bool {
    state::shadow_stack_capacity() != 0
}

pub fn current_mode() -> CfiMode {
    if state::shadow_stack_enabled() {
        CfiMode::ShadowStack
    } else {
        CfiMode::Disabled
    }
}

pub fn software_shadow_stack_push(ra: usize) -> Result<(), CfiError> {
    state::push_shadow_return(ra)
}

pub fn software_shadow_stack_pop() -> Result<usize, CfiError> {
    state::pop_shadow_return()
}

pub fn software_shadow_stack_check(expected: usize, actual: usize) -> bool {
    expected == actual
}
