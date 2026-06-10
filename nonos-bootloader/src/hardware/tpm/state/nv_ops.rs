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

use super::core::TpmState;
use crate::hardware::tpm::nv::{nv_read_impl, nv_write_impl};
use crate::hardware::tpm::types::{NvIndex, TpmError};

impl TpmState {
    pub fn nv_read(&self, index: &NvIndex, buf: &mut [u8]) -> Result<usize, TpmError> {
        nv_read_impl(self, index, buf)
    }
    pub fn nv_write(&self, index: &NvIndex, data: &[u8]) -> Result<(), TpmError> {
        nv_write_impl(self, index, data)
    }
}
