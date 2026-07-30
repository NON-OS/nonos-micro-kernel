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

pub mod api;
pub mod canary;
pub mod containers;
pub mod erase;
pub mod guard;
mod kernel_stacks;
pub mod primitives;
pub mod state;
pub mod types;
mod user_range;

pub use api::{
    get_level, init, on_free, on_realloc, sanitization_stats, sanitize_process_memory, set_level,
    zerostate_shutdown_wipe,
};
pub use canary::{get_stack_canary, init_stack_canary, stack_canary_failed, verify_stack_canary};
pub use containers::{SecureString, SensitiveData};
pub use erase::{
    dod_5220_erase, gutmann_erase, paranoid_erase, sanitize, sanitize_slice, secure_zero,
    secure_zero_slice,
};
pub use guard::{allocate_with_guards, free_with_guards, GuardPage};
pub use types::{SanitizationLevel, SanitizationStats, StackCanaryConfig};
