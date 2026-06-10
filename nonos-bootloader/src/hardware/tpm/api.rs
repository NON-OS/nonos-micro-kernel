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

use super::state::TpmState;
use super::types::{NvIndex, TpmError};
use spin::Mutex;

pub static TPM: Mutex<TpmState> = Mutex::new(TpmState::new());

pub fn init_tpm() -> Result<bool, TpmError> {
    TPM.lock().detect()
}
pub fn is_tpm_available() -> bool {
    TPM.lock().initialized
}
pub fn nv_read(index: &NvIndex, buf: &mut [u8]) -> Result<usize, TpmError> {
    TPM.lock().nv_read(index, buf)
}
pub fn nv_write(index: &NvIndex, data: &[u8]) -> Result<(), TpmError> {
    TPM.lock().nv_write(index, data)
}
pub fn pcr_extend(pcr_index: u32, digest: &[u8; 32]) -> Result<(), TpmError> {
    TPM.lock().pcr_extend(pcr_index, digest)
}

pub fn get_tpm_ek_public(
    _st: &uefi::prelude::SystemTable<uefi::prelude::Boot>,
) -> Result<alloc::vec::Vec<u8>, &'static str> {
    let tpm = TPM.lock();
    if !tpm.initialized {
        return Err("TPM not initialized");
    }
    tpm.get_ek_public()
}
