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

extern crate alloc;
use alloc::string::ToString;

use crate::capabilities::*;
use crate::test::framework::TestResult;

pub(crate) fn test_capability_bit_values() -> TestResult {
    if Capability::CoreExec.bit() != 1 {
        return TestResult::Fail;
    }
    if Capability::IO.bit() != 2 {
        return TestResult::Fail;
    }
    if Capability::Network.bit() != 4 {
        return TestResult::Fail;
    }
    if Capability::IPC.bit() != 8 {
        return TestResult::Fail;
    }
    if Capability::Memory.bit() != 16 {
        return TestResult::Fail;
    }
    if Capability::Crypto.bit() != 32 {
        return TestResult::Fail;
    }
    if Capability::FileSystem.bit() != 64 {
        return TestResult::Fail;
    }
    if Capability::Hardware.bit() != 128 {
        return TestResult::Fail;
    }
    if Capability::Debug.bit() != 256 {
        return TestResult::Fail;
    }
    if Capability::Admin.bit() != 512 {
        return TestResult::Fail;
    }
    if Capability::RegisterService.bit() != 1024 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_bits_are_powers_of_two() -> TestResult {
    for cap in Capability::all() {
        let bit = cap.bit();
        if !bit.is_power_of_two() {
            return TestResult::Fail;
        }
    }
    TestResult::Pass
}

pub(crate) fn test_capability_bits_are_unique() -> TestResult {
    let all = Capability::all();
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            if all[i].bit() == all[j].bit() {
                return TestResult::Fail;
            }
        }
    }
    TestResult::Pass
}

pub(crate) fn test_capability_all_returns_22_items() -> TestResult {
    if Capability::all().len() != 22 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_count_matches_all_len() -> TestResult {
    if Capability::count() != Capability::all().len() {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_core_exec() -> TestResult {
    if Capability::CoreExec.as_str() != "CoreExec" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_io() -> TestResult {
    if Capability::IO.as_str() != "IO" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_network() -> TestResult {
    if Capability::Network.as_str() != "Network" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_ipc() -> TestResult {
    if Capability::IPC.as_str() != "IPC" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_memory() -> TestResult {
    if Capability::Memory.as_str() != "Memory" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_crypto() -> TestResult {
    if Capability::Crypto.as_str() != "Crypto" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_filesystem() -> TestResult {
    if Capability::FileSystem.as_str() != "FileSystem" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_hardware() -> TestResult {
    if Capability::Hardware.as_str() != "Hardware" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_debug() -> TestResult {
    if Capability::Debug.as_str() != "Debug" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_admin() -> TestResult {
    if Capability::Admin.as_str() != "Admin" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_as_str_register_service() -> TestResult {
    if Capability::RegisterService.as_str() != "RegisterService" {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_display_matches_as_str() -> TestResult {
    for cap in Capability::all() {
        if cap.to_string() != cap.as_str() {
            return TestResult::Fail;
        }
    }
    TestResult::Pass
}

pub(crate) fn test_capability_clone() -> TestResult {
    let cap = Capability::Admin;
    let cloned = cap.clone();
    if cap != cloned {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_copy() -> TestResult {
    let cap = Capability::Network;
    let copied: Capability = cap;
    if cap != copied {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_equality() -> TestResult {
    if Capability::Admin != Capability::Admin {
        return TestResult::Fail;
    }
    if Capability::Admin == Capability::Debug {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_capability_debug_format() -> TestResult {
    let cap = Capability::Memory;
    let debug_str = alloc::format!("{:?}", cap);
    if !debug_str.contains("Memory") {
        return TestResult::Fail;
    }
    TestResult::Pass
}
