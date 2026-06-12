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

use uefi::prelude::*;

use crate::kernel_verify::CryptoVerifyResult;
use crate::menu::SecurityMode;

use super::error::{handle_invalid_signature, handle_no_signature};
use super::valid::handle_valid_signature;

pub fn verify_signature(
    st: &mut SystemTable<Boot>,
    res: &CryptoVerifyResult,
    mode: SecurityMode,
    gop: bool,
) {
    if !res.signature_present {
        handle_no_signature(st, mode, gop);
    } else if !res.signature_valid {
        handle_invalid_signature(st, mode, gop);
    } else {
        handle_valid_signature(gop);
    }
}
