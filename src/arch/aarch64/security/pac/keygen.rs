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

use core::arch::asm;

use crate::arch::aarch64::cpu::features::{has_feature, CpuFeature};
use crate::crypto::rng::fill_random_bytes_secure;

use super::error::{PacError, PacResult};
use super::key::{PacKey, PacKeys};

pub fn generate_keys() -> PacResult<PacKeys> {
    Ok(PacKeys {
        ia: generate_key()?,
        ib: generate_key()?,
        da: generate_key()?,
        db: generate_key()?,
        ga: generate_key()?,
    })
}

fn generate_key() -> PacResult<PacKey> {
    if has_feature(CpuFeature::Rng) {
        Ok(PacKey::new(read_rndr(), read_rndr()))
    } else {
        secure_rng_key()
    }
}

fn read_rndr() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {}, rndr", out(reg) value);
    }
    value
}

fn secure_rng_key() -> PacResult<PacKey> {
    let mut bytes = [0u8; 16];
    fill_random_bytes_secure(&mut bytes).map_err(|_| PacError::EntropyUnavailable)?;
    Ok(PacKey::from_bytes(bytes))
}
