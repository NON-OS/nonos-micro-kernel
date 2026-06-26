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

use super::capability::Tcg2BootServiceCapability;
use uefi::proto::unsafe_protocol;
use uefi::Status;

#[repr(C)]
#[unsafe_protocol("607f766c-7455-42be-930b-e4d76db2720f")]
pub struct Tcg2Protocol {
    pub get_capability:
        unsafe extern "efiapi" fn(*mut Tcg2Protocol, *mut Tcg2BootServiceCapability) -> Status,
    pub get_event_log:
        unsafe extern "efiapi" fn(*mut Tcg2Protocol, u32, *mut u64, *mut u64, *mut bool) -> Status,
    pub hash_log_extend_event:
        unsafe extern "efiapi" fn(*mut Tcg2Protocol, u64, *const u8, u64, *const u8) -> Status,
    pub submit_command:
        unsafe extern "efiapi" fn(*mut Tcg2Protocol, u32, *const u8, u32, *mut u8) -> Status,
    pub get_active_pcr_banks: unsafe extern "efiapi" fn(*mut Tcg2Protocol, *mut u32) -> Status,
    pub set_active_pcr_banks: unsafe extern "efiapi" fn(*mut Tcg2Protocol, u32) -> Status,
    pub get_result_of_set_active_pcr_banks:
        unsafe extern "efiapi" fn(*mut Tcg2Protocol, *mut u32, *mut u32) -> Status,
}
