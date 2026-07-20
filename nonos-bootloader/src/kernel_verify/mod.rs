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

mod delay;
mod display;
mod display_status;
mod elf;
mod footer;
mod hash;
mod helpers;
mod key;
mod signature;
mod signature_display;
mod signature_ed25519;
mod signature_hybrid;
mod signature_message;
mod signature_passed;
mod signature_policy;
mod size;
#[cfg(feature = "stark-kernel-attest")]
mod stark_attest;
mod types;
mod verify;
mod verify_error;

pub use delay::mini_delay;
pub use display::{byte_to_hex, print, print_hex_bytes, print_hex_char};
pub use display_status::{
    print_kernel_size, print_verification_failure, print_verification_success,
};
pub use footer::handle_missing_footer;
pub use types::{CryptoVerifyResult, MIN_KERNEL_SIZE, SIGNATURE_SIZE};
pub use verify::verify_kernel_crypto;
