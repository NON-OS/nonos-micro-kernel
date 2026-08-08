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

use crate::hardware::tpm::commands::{pcr_extend_impl, receive_response_impl, send_command_impl};
use crate::hardware::tpm::constants::TPM_MMIO_BASE;
use crate::hardware::tpm::types::TpmError;

pub struct TpmState {
    pub(crate) base: u64,
    pub initialized: bool,
    pub(crate) version: u8,
    /// Which register file the part presents. The two are not interchangeable:
    /// the same offsets carry different meanings and locality is requested
    /// through different registers, so an access has to know which it is
    /// talking to.
    pub(crate) is_crb: bool,
}

impl TpmState {
    pub const fn new() -> Self {
        Self { base: TPM_MMIO_BASE, initialized: false, version: 0, is_crb: false }
    }
    pub fn send_command(&self, cmd: &[u8]) -> Result<(), TpmError> {
        send_command_impl(self, cmd)
    }
    pub fn receive_response(&self, buf: &mut [u8]) -> Result<usize, TpmError> {
        receive_response_impl(self, buf)
    }
    pub fn pcr_extend(&self, pcr_index: u32, digest: &[u8; 32]) -> Result<(), TpmError> {
        pcr_extend_impl(self, pcr_index, digest)
    }
}
