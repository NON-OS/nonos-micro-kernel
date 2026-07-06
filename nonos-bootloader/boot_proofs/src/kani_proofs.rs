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

//! Kani harnesses: the anti-rollback invariants hold for every u64 version and
//! floor, not just the sampled ones in the runnable tests.

use crate::image_format::parse::parse_image_footer;
use crate::security::anti_rollback::AntiRollbackState;

fn active(minimum_kernel: u64) -> AntiRollbackState {
    let mut s = AntiRollbackState::new();
    s.initialized = true;
    s.tpm_available = true;
    s.state.minimum_kernel = minimum_kernel;
    s
}

// A version is accepted if and only if it is non-zero and at least the floor.
#[kani::proof]
fn check_accepts_exactly_the_valid_versions() {
    let minimum: u64 = kani::any();
    let version: u64 = kani::any();
    let s = active(minimum);
    match s.check_kernel_version(version) {
        Ok(()) => {
            assert!(version != 0);
            assert!(version >= minimum);
        }
        Err(_) => {
            assert!(version == 0 || version < minimum);
        }
    }
}

// No update can ever lower the floor, whatever version is presented.
#[kani::proof]
fn update_never_lowers_the_floor() {
    let minimum: u64 = kani::any();
    let version: u64 = kani::any();
    let timestamp: u64 = kani::any();
    let mut s = active(minimum);
    let _ = s.update_kernel_version(version, timestamp);
    assert!(s.state.minimum_kernel >= minimum);
}

// Once a version boots, every strictly older version is rejected forever.
#[kani::proof]
fn no_rollback_after_a_successful_boot() {
    let minimum: u64 = kani::any();
    let version: u64 = kani::any();
    let mut s = active(minimum);
    if s.update_kernel_version(version, 0).is_ok() {
        let older: u64 = kani::any();
        kani::assume(older < version);
        assert!(s.check_kernel_version(older).is_err());
    }
}

// Parsing any attacker-controlled image footer is free of panics, out-of-bounds
// access and arithmetic overflow. Kani verifies these for every byte pattern of
// this size; the region-extraction slices cannot escape the buffer.
#[kani::proof]
#[kani::unwind(4)]
fn parse_footer_is_total_and_in_bounds() {
    let buf: [u8; 72] = kani::any();
    let _ = parse_image_footer(&buf);
}
