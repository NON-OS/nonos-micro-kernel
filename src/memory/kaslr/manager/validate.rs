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

use super::super::constants::*;
use super::super::error::{KaslrError, KaslrResult};
use super::derive::derive_subkey;
use super::init::{boot_nonce, get_slide};
use crate::memory::layout;

pub fn validate() -> KaslrResult<()> {
    let slide = get_slide();
    let nonce = boot_nonce()?;

    if slide == 0 && nonce == 0 {
        return Err(KaslrError::NotInitialized);
    }
    if slide % (layout::PAGE_SIZE as u64) != 0 {
        return Err(KaslrError::SlideNotAligned);
    }
    if slide < SAFE_SLIDE_MIN || slide > SAFE_SLIDE_MAX {
        return Err(KaslrError::SlideOutOfRange);
    }
    Ok(())
}

pub fn verify_slide_integrity() -> bool {
    if validate().is_err() {
        return false;
    }

    let current_slide = get_slide();
    let expected_layout_base = layout::KERNEL_BASE + current_slide;
    let actual_layout_base = layout::KERNEL_BASE + current_slide;
    if actual_layout_base != expected_layout_base {
        return false;
    }

    let nonce = match boot_nonce() {
        Ok(n) => n,
        Err(_) => return false,
    };
    if nonce == 0 {
        return false;
    }

    let mut integrity_buffer = [0u8; INTEGRITY_CHECK_BUFFER_SIZE];
    if derive_subkey(INTEGRITY_CHECK_LABEL, &mut integrity_buffer).is_err() {
        return false;
    }

    for byte in integrity_buffer.iter() {
        if *byte == 0 {
            return false;
        }
    }

    let entropy_check = integrity_buffer.iter().fold(0u8, |acc, &x| acc ^ x);
    if entropy_check == 0 || entropy_check == 0xFF {
        return false;
    }

    true
}
