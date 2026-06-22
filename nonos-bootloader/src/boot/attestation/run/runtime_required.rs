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

use super::failed::handle_verification_failed;
use crate::menu::SecurityMode;
use crate::zk::BootAttestationResult;
use uefi::prelude::*;

pub fn handle_runtime_required(
    st: &mut SystemTable<Boot>,
    gop: bool,
    mode: SecurityMode,
) -> BootAttestationResult {
    let result = BootAttestationResult::verification_failed(
        [0u8; 32],
        [0u8; 32],
        "runtime ZK sidecar required",
    );
    handle_verification_failed(st, &result, gop, mode)
}
