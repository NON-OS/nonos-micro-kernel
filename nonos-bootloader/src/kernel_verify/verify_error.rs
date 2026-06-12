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

use uefi::cstr16;
use uefi::prelude::*;

use super::display::print;
use crate::crypto::sig::VerifyError;

pub fn display_verification_error(e: VerifyError, st: &mut SystemTable<Boot>) {
    match e {
        VerifyError::InvalidSignature => {
            print(st, cstr16!("  [CRYPTO] ERROR: Signature does not match any trusted key\r\n"));
        }
        VerifyError::KeyNotFound => {
            print(st, cstr16!("  [CRYPTO] ERROR: Signing key not in trusted keystore\r\n"));
        }
        VerifyError::NotInitialized => {
            print(st, cstr16!("  [CRYPTO] ERROR: Keystore not initialized\r\n"));
        }
        VerifyError::MalformedSignature => {
            print(st, cstr16!("  [CRYPTO] ERROR: Malformed signature data\r\n"));
        }
        VerifyError::Bounds => {
            print(st, cstr16!("  [CRYPTO] ERROR: Signature bounds error\r\n"));
        }
        VerifyError::KeyRevoked => {
            print(st, cstr16!("  [CRYPTO] ERROR: Signing key has been revoked\r\n"));
        }
        VerifyError::KeyVersionTooOld => {
            print(st, cstr16!("  [CRYPTO] ERROR: Key version below minimum required\r\n"));
        }
    }
}
