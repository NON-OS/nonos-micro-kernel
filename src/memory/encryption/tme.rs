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

use super::error::{MemEncryptionError, MemEncryptionResult};
use super::types::EncryptionCapability;

const MSR_IA32_TME_CAPABILITY: u32 = 0x981;
const MSR_IA32_TME_ACTIVATE: u32 = 0x982;
const MSR_IA32_MKTME_KEYID_PARTITIONING: u32 = 0x87;

const TME_ENABLE_BIT: u64 = 1 << 1;
const TME_LOCKED_BIT: u64 = 1 << 0;

pub fn init_tme(cap: &EncryptionCapability) -> MemEncryptionResult<()> {
    if !cap.tme_supported {
        return Err(MemEncryptionError::NotSupported);
    }
    let tme_cap = rdmsr(MSR_IA32_TME_CAPABILITY);
    if tme_cap == 0 {
        return Err(MemEncryptionError::HardwareError);
    }
    Ok(())
}

pub fn enable_tme(_cap: &EncryptionCapability) -> MemEncryptionResult<()> {
    let activate = rdmsr(MSR_IA32_TME_ACTIVATE);
    if (activate & TME_LOCKED_BIT) != 0 {
        if (activate & TME_ENABLE_BIT) != 0 {
            return Ok(());
        }
        // Firmware locked the activate MSR with TME off.
        return Err(MemEncryptionError::NotSupported);
    }
    let new_value = activate | TME_ENABLE_BIT | TME_LOCKED_BIT;
    wrmsr(MSR_IA32_TME_ACTIVATE, new_value);
    Ok(())
}

pub fn get_tme_keyid_bits(cap: &EncryptionCapability) -> u8 {
    cap.keyid_bits
}

pub fn get_mktme_keyid_partitioning() -> (u32, u32) {
    let partitioning = rdmsr(MSR_IA32_MKTME_KEYID_PARTITIONING);
    let num_tme_keys = (partitioning & 0xFFFF) as u32;
    let num_mktme_keys = ((partitioning >> 16) & 0xFFFF) as u32;
    (num_tme_keys, num_mktme_keys)
}

pub fn is_tme_enabled() -> bool {
    let activate = rdmsr(MSR_IA32_TME_ACTIVATE);
    (activate & TME_ENABLE_BIT) != 0
}

fn rdmsr(msr: u32) -> u64 {
    let (low, high): (u32, u32);
    unsafe {
        core::arch::asm!("rdmsr", in("ecx") msr, out("eax") low, out("edx") high);
    }
    ((high as u64) << 32) | (low as u64)
}

fn wrmsr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    unsafe {
        core::arch::asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high);
    }
}
