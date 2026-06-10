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

use uefi::table::boot::SearchType;
use uefi::Identify;

use crate::security::tpm_types::{Tcg2EventHeader, Tcg2Protocol, EV_POST_CODE};

pub fn locate_tcg2_protocol(bs: &uefi::table::boot::BootServices) -> Option<*mut Tcg2Protocol> {
    let handles = bs.locate_handle_buffer(SearchType::ByProtocol(&Tcg2Protocol::GUID)).ok()?;
    let handle = handles.first()?;
    let protocol = bs.open_protocol_exclusive::<Tcg2Protocol>(*handle).ok()?;
    let ptr = &*protocol as *const Tcg2Protocol as *mut Tcg2Protocol;
    core::mem::forget(protocol);
    Some(ptr)
}

pub fn extend_pcr_via_tcg2(
    tcg2: *mut Tcg2Protocol,
    pcr_index: u32,
    digest: &[u8; 32],
) -> Result<(), &'static str> {
    if tcg2.is_null() {
        return Err("null TCG2 handle");
    }
    let header = Tcg2EventHeader {
        header_size: core::mem::size_of::<Tcg2EventHeader>() as u32,
        header_version: 1,
        pcr_index,
        event_type: EV_POST_CODE,
    };
    const PE_COFF_IMAGE: u64 = 0x10;
    unsafe {
        let status = ((*tcg2).hash_log_extend_event)(
            tcg2,
            PE_COFF_IMAGE,
            digest.as_ptr(),
            digest.len() as u64,
            &header,
        );
        if status.is_success() {
            Ok(())
        } else {
            Err("TCG2 extend failed")
        }
    }
}
