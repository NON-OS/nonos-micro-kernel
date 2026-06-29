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

pub mod attestation;
pub mod crypto;
pub mod elf;
pub mod hardware;
pub mod kernel;
pub mod prepare;
pub mod security;
pub mod shell;
pub mod uefi;
pub mod util;
pub mod zk_challenge;
pub mod zk_init;

pub use attestation::run_zk_attestation;
pub use crypto::{commit_rollback, run_crypto_verification};
pub use elf::run_elf_parse;
pub use hardware::run_hardware_discovery;
pub use kernel::run_kernel_load;
pub use prepare::{run_handoff_prepare, HandoffParams};
pub use security::{enforce_policy, run_security_checks};
pub use shell::exit_to_shell;
pub use uefi::{run_boot_screen_init, run_uefi_init};
pub use util::{fatal_reset, micro_delay, mini_delay, print_u64};
pub use zk_init::initialize_zk_replay_protection;
