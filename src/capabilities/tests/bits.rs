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

use crate::capabilities::*;
use crate::test::framework::TestResult;

pub(crate) fn test_caps_to_bits_empty() -> TestResult {
    if caps_to_bits(&[]) != 0 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_caps_to_bits_single() -> TestResult {
    if caps_to_bits(&[Capability::CoreExec]) != 1 {
        return TestResult::Fail;
    }
    if caps_to_bits(&[Capability::IO]) != 2 {
        return TestResult::Fail;
    }
    if caps_to_bits(&[Capability::Admin]) != 512 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_caps_to_bits_multiple() -> TestResult {
    let caps = [Capability::CoreExec, Capability::IO, Capability::Network];
    if caps_to_bits(&caps) != 1 | 2 | 4 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_caps_to_bits_all() -> TestResult {
    let all = Capability::all();
    let bits = caps_to_bits(&all);
    let mut expected = 0;
    for cap in all {
        expected |= cap.bit();
    }
    if bits != expected {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_caps_to_bits_duplicates() -> TestResult {
    let caps = [Capability::Admin, Capability::Admin, Capability::Admin];
    if caps_to_bits(&caps) != 512 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_bits_to_caps_zero() -> TestResult {
    let caps = bits_to_caps(0);
    if !caps.is_empty() {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_bits_to_caps_single() -> TestResult {
    let caps = bits_to_caps(1);
    if caps.len() != 1 {
        return TestResult::Fail;
    }
    if caps[0] != Capability::CoreExec {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_bits_to_caps_multiple() -> TestResult {
    let caps = bits_to_caps(1 | 2 | 4);
    if caps.len() != 3 {
        return TestResult::Fail;
    }
    if !caps.contains(&Capability::CoreExec) {
        return TestResult::Fail;
    }
    if !caps.contains(&Capability::IO) {
        return TestResult::Fail;
    }
    if !caps.contains(&Capability::Network) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_bits_to_caps_all() -> TestResult {
    let all = Capability::all();
    let caps = bits_to_caps(caps_to_bits(&all));
    if caps.len() != all.len() {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_bits_to_caps_ignores_invalid_bits() -> TestResult {
    let caps = bits_to_caps(1 | (1 << 40));
    if caps.len() != 1 {
        return TestResult::Fail;
    }
    if caps[0] != Capability::CoreExec {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_roundtrip_caps_to_bits_to_caps() -> TestResult {
    let original = [Capability::Admin, Capability::Crypto, Capability::Memory];
    let bits = caps_to_bits(&original);
    let recovered = bits_to_caps(bits);
    if recovered.len() != 3 {
        return TestResult::Fail;
    }
    for cap in &original {
        if !recovered.contains(cap) {
            return TestResult::Fail;
        }
    }
    TestResult::Pass
}

pub(crate) fn test_has_capability_true() -> TestResult {
    let bits = caps_to_bits(&[Capability::Admin, Capability::Debug]);
    if !has_capability(bits, Capability::Admin) {
        return TestResult::Fail;
    }
    if !has_capability(bits, Capability::Debug) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_has_capability_false() -> TestResult {
    let bits = caps_to_bits(&[Capability::Admin]);
    if has_capability(bits, Capability::Debug) {
        return TestResult::Fail;
    }
    if has_capability(bits, Capability::Network) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_has_capability_zero_bits() -> TestResult {
    if has_capability(0, Capability::Admin) {
        return TestResult::Fail;
    }
    if has_capability(0, Capability::CoreExec) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_add_capability_to_zero() -> TestResult {
    let bits = add_capability(0, Capability::Admin);
    if bits != 512 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_add_capability_cumulative() -> TestResult {
    let mut bits = 0;
    bits = add_capability(bits, Capability::CoreExec);
    bits = add_capability(bits, Capability::IO);
    bits = add_capability(bits, Capability::Network);
    if bits != 7 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_add_capability_idempotent() -> TestResult {
    let bits = add_capability(512, Capability::Admin);
    if bits != 512 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_remove_capability_present() -> TestResult {
    let bits = caps_to_bits(&[Capability::Admin, Capability::Debug]);
    let after = remove_capability(bits, Capability::Admin);
    if has_capability(after, Capability::Admin) {
        return TestResult::Fail;
    }
    if !has_capability(after, Capability::Debug) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_remove_capability_not_present() -> TestResult {
    let bits = caps_to_bits(&[Capability::Admin]);
    let after = remove_capability(bits, Capability::Debug);
    if bits != after {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_remove_capability_from_zero() -> TestResult {
    let after = remove_capability(0, Capability::Admin);
    if after != 0 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_remove_all_capabilities() -> TestResult {
    let mut bits = caps_to_bits(&Capability::all());
    for cap in Capability::all() {
        bits = remove_capability(bits, cap);
    }
    if bits != 0 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_count_zero() -> TestResult {
    if capability_count(0) != 0 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_count_one() -> TestResult {
    if capability_count(1) != 1 {
        return TestResult::Fail;
    }
    if capability_count(512) != 1 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_count_multiple() -> TestResult {
    let bits = caps_to_bits(&[Capability::Admin, Capability::Debug, Capability::Crypto]);
    if capability_count(bits) != 3 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_count_all() -> TestResult {
    let bits = caps_to_bits(&Capability::all());
    if capability_count(bits) != 11 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_count_ignores_high_bits() -> TestResult {
    let bits = 1 | (1 << 50);
    if capability_count(bits) != 2 {
        return TestResult::Fail;
    }
    TestResult::Pass
}
