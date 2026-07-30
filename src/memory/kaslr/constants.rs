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

//! KASLR Constants
//!
//! Named constants for Kernel Address Space Layout Randomization.

/// Default KASLR window size (1 GiB)
pub const DEFAULT_WINDOW_SIZE: u64 = 0x40000000;

/// Minimum KASLR slide (256 MiB)
pub const MIN_SLIDE: u64 = 0x10000000;

/// Maximum KASLR slide (2 GiB)
pub const MAX_SLIDE: u64 = 0x80000000;

/// Minimum safe slide value for validation (16 MiB)
pub const SAFE_SLIDE_MIN: u64 = 0x1000000;

/// Maximum safe slide value for validation (4 GiB)
pub const SAFE_SLIDE_MAX: u64 = 0x100000000;

/// Initial entropy pool seed
pub const INITIAL_ENTROPY_SEED: u64 = 0x1337DEADBEEF4242;

/// Entropy mixing multiplier (prime constant)
pub const ENTROPY_MIX_MULTIPLIER: u64 = 0x2545f4914f6cdd1d;

/// Nonce generation multiplier (golden ratio-derived)
pub const NONCE_GEN_MULTIPLIER: u64 = 0x9e3779b97f4a7c15;

/// Nonce rotation bits
pub const NONCE_ROTATE_BITS: u32 = 23;

/// Entropy collection spin loop iterations
pub const ENTROPY_SPIN_ITERATIONS: usize = 1000;

/// KDF label prefix for subkey derivation
pub const KDF_LABEL_PREFIX: &[u8] = b"NONOS-KASLR-KDF:";

/// Hash output size (SHA3-256)
pub const HASH_OUTPUT_SIZE: usize = 32;

/// Integrity check label
pub const INTEGRITY_CHECK_LABEL: &[u8] = b"integrity_check";

/// Integrity check buffer size
pub const INTEGRITY_CHECK_BUFFER_SIZE: usize = 64;
