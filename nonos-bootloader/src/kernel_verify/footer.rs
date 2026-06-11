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

use super::helpers::compute_and_display_hash;
use super::types::CryptoVerifyResult;
use crate::log::logger::log_info;

pub fn handle_missing_footer(
    kernel_data: &[u8],
    result: &mut CryptoVerifyResult,
    st: &mut SystemTable<Boot>,
) {
    log_info("kernel_verify", "No production footer - computing raw hash");
    result.kernel_code_size = kernel_data.len();
    result.signature_present = false;
    compute_and_display_hash(kernel_data, result, st);
}
