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

use crate::security::anti_rollback::types::RollbackError;
use crate::security::anti_rollback::AntiRollbackState;

// An initialized, TPM-backed state with a given minimum kernel version. This is
// the configuration in which the anti-rollback floor is enforced.
fn active(minimum_kernel: u64) -> AntiRollbackState {
    let mut s = AntiRollbackState::new();
    s.initialized = true;
    s.tpm_available = true;
    s.state.minimum_kernel = minimum_kernel;
    s
}

#[test]
fn version_zero_is_always_invalid() {
    let s = active(5);
    assert!(matches!(s.check_kernel_version(0), Err(RollbackError::InvalidVersion)));
}

#[test]
fn without_an_anchor_every_version_is_rejected() {
    // Neither initialized nor TPM-backed: there is no trusted floor, so nothing
    // may boot.
    let s = AntiRollbackState::new();
    assert!(matches!(s.check_kernel_version(10), Err(RollbackError::TpmNotAvailable)));
}

#[test]
fn below_the_floor_is_rejected_at_or_above_is_accepted() {
    let s = active(10);
    assert!(matches!(
        s.check_kernel_version(9),
        Err(RollbackError::KernelVersionTooOld { .. })
    ));
    assert!(s.check_kernel_version(10).is_ok());
    assert!(s.check_kernel_version(11).is_ok());
}

#[test]
fn booting_a_version_raises_the_floor_and_blocks_older() {
    let mut s = active(10);
    s.update_kernel_version(20, 111).unwrap();
    assert_eq!(s.state.minimum_kernel, 20, "floor raised to the booted version");
    assert!(s.state.kernel_version >= 20);
    // No rollback: anything below the new floor is now rejected.
    assert!(matches!(
        s.check_kernel_version(19),
        Err(RollbackError::KernelVersionTooOld { .. })
    ));
    assert!(s.check_kernel_version(20).is_ok());
}

#[test]
fn a_too_old_boot_is_rejected_and_leaves_state_untouched() {
    let mut s = active(50);
    let min_before = s.state.minimum_kernel;
    let count_before = s.state.boot_count;
    assert!(s.update_kernel_version(40, 1).is_err());
    assert_eq!(s.state.minimum_kernel, min_before, "floor unchanged");
    assert_eq!(s.state.boot_count, count_before, "no boot recorded");
}

#[test]
fn the_floor_never_decreases_across_updates() {
    let mut s = active(10);
    s.update_kernel_version(30, 1).unwrap();
    s.update_kernel_version(30, 2).unwrap();
    assert_eq!(s.state.minimum_kernel, 30);
    // A lower version can never be booted again.
    assert!(s.update_kernel_version(25, 3).is_err());
    assert_eq!(s.state.minimum_kernel, 30);
    // Only the two accepted boots were counted.
    assert_eq!(s.state.boot_count, 2);
}

#[test]
fn setting_the_minimum_only_ever_raises_it() {
    let mut s = active(10);
    s.set_minimum_kernel_version(5).unwrap();
    assert_eq!(s.state.minimum_kernel, 10, "a lower minimum is ignored");
    s.set_minimum_kernel_version(15).unwrap();
    assert_eq!(s.state.minimum_kernel, 15, "a higher minimum is applied");
}
