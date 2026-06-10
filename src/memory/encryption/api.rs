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

use super::cbit_validate::validate_c_bit_position;
use super::detect::detect_encryption_support;
use super::error::{MemEncryptionError, MemEncryptionResult};
use super::sme::{enable_sme, init_sme, sme_encrypt_page};
use super::tme::{enable_tme, init_tme};
use super::types::{EncryptionCapability, EncryptionStatus, MemEncryption};
use crate::memory::addr::PhysAddr;
use core::sync::atomic::Ordering;
use spin::Once;

static ENCRYPTION_STATUS: EncryptionStatus = EncryptionStatus::new();
static ENCRYPTION_CAP: Once<EncryptionCapability> = Once::new();

pub fn init_memory_encryption() -> MemEncryptionResult<MemEncryption> {
    let cap = ENCRYPTION_CAP.call_once(detect_encryption_support);
    let enc_type = cap.best_available();
    match enc_type {
        MemEncryption::AmdSme | MemEncryption::AmdSev => {
            // Firmware left SME off: plaintext is the configured state,
            // not a fault. Anything else propagates.
            match init_sme(cap) {
                Ok(()) => {}
                Err(MemEncryptionError::NotSupported) => return Ok(MemEncryption::None),
                Err(e) => return Err(e),
            }
            let mask = enable_sme(cap)?;
            validate_c_bit_position(cap.c_bit_position)?;
            ENCRYPTION_STATUS.c_bit_mask.store(mask, Ordering::Release);
            ENCRYPTION_STATUS.enabled.store(false, Ordering::Release);
            Ok(MemEncryption::Pending(cap.c_bit_position))
        }
        MemEncryption::IntelTme | MemEncryption::IntelMktme => {
            init_tme(cap)?;
            enable_tme(cap)?;
            ENCRYPTION_STATUS.enabled.store(true, Ordering::Release);
            Ok(enc_type)
        }
        MemEncryption::None => Ok(MemEncryption::None),
        MemEncryption::Pending(_) => Ok(MemEncryption::None),
    }
}

pub fn is_encryption_enabled() -> bool {
    ENCRYPTION_STATUS.enabled.load(Ordering::Acquire)
}

pub fn encrypt_region(phys_start: PhysAddr, _size: usize) -> MemEncryptionResult<PhysAddr> {
    if !is_encryption_enabled() {
        return Err(MemEncryptionError::NotSupported);
    }
    let mask = ENCRYPTION_STATUS.c_bit_mask.load(Ordering::Acquire);
    if mask != 0 {
        ENCRYPTION_STATUS.pages_encrypted.fetch_add(1, Ordering::Relaxed);
        Ok(sme_encrypt_page(phys_start, mask))
    } else {
        Ok(phys_start)
    }
}

pub fn decrypt_region(phys_start: PhysAddr, _size: usize) -> MemEncryptionResult<PhysAddr> {
    if !is_encryption_enabled() {
        return Err(MemEncryptionError::NotSupported);
    }
    let mask = ENCRYPTION_STATUS.c_bit_mask.load(Ordering::Acquire);
    if mask != 0 {
        Ok(PhysAddr::new(phys_start.as_u64() & !mask))
    } else {
        Ok(phys_start)
    }
}

pub fn get_encryption_stats() -> (u64, bool) {
    (ENCRYPTION_STATUS.pages_encrypted.load(Ordering::Relaxed), is_encryption_enabled())
}
