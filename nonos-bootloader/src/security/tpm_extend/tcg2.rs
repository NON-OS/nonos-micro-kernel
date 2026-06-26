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

use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams, SearchType};
use uefi::Identify;

use crate::security::tpm_types::{Tcg2Event, Tcg2EventHeader, Tcg2Protocol, EV_POST_CODE};

pub fn extend_pcr_via_tcg2(
    bs: &uefi::table::boot::BootServices,
    pcr_index: u32,
    digest: &[u8; 32],
) -> Result<(), &'static str> {
    let handles = bs
        .locate_handle_buffer(SearchType::ByProtocol(&Tcg2Protocol::GUID))
        .map_err(|_| "no TCG2 handle")?;
    let handle = *handles.first().ok_or("no TCG2 protocol")?;
    let proto = unsafe {
        bs.open_protocol::<Tcg2Protocol>(
            OpenProtocolParams { handle, agent: bs.image_handle(), controller: None },
            OpenProtocolAttributes::GetProtocol,
        )
    }
    .map_err(|_| "open TCG2 failed")?;
    let tcg2 = &*proto as *const Tcg2Protocol as *mut Tcg2Protocol;
    let event = Tcg2Event {
        size: core::mem::size_of::<Tcg2Event>() as u32,
        header: Tcg2EventHeader {
            header_size: core::mem::size_of::<Tcg2EventHeader>() as u32,
            header_version: 1,
            pcr_index,
            event_type: EV_POST_CODE,
        },
        event: *digest,
    };
    let status = unsafe {
        ((*tcg2).hash_log_extend_event)(
            tcg2,
            0,
            digest.as_ptr(),
            digest.len() as u64,
            &event as *const Tcg2Event as *const u8,
        )
    };
    drop(proto);
    if status.is_success() {
        Ok(())
    } else {
        Err("TCG2 extend failed")
    }
}

pub fn submit_tpm_command(
    bs: &uefi::table::boot::BootServices,
    cmd: &[u8],
    response: &mut [u8],
) -> Result<usize, &'static str> {
    let handles = bs
        .locate_handle_buffer(SearchType::ByProtocol(&Tcg2Protocol::GUID))
        .map_err(|_| "no TCG2 handle")?;
    let handle = *handles.first().ok_or("no TCG2 protocol")?;
    // OVMF keeps the TCG2 protocol open BY_DRIVER, so an exclusive open is
    // denied. GetProtocol borrows the interface without disturbing that owner.
    let proto = unsafe {
        bs.open_protocol::<Tcg2Protocol>(
            OpenProtocolParams { handle, agent: bs.image_handle(), controller: None },
            OpenProtocolAttributes::GetProtocol,
        )
    }
    .map_err(|_| "open TCG2 failed")?;
    let tcg2 = &*proto as *const Tcg2Protocol as *mut Tcg2Protocol;
    let status = unsafe {
        ((*tcg2).submit_command)(
            tcg2,
            cmd.len() as u32,
            cmd.as_ptr(),
            response.len() as u32,
            response.as_mut_ptr(),
        )
    };
    drop(proto);
    if !status.is_success() {
        return Err("submit_command failed");
    }
    if response.len() < 10 {
        return Err("short response");
    }
    let size = u32::from_be_bytes([response[2], response[3], response[4], response[5]]);
    Ok(size as usize)
}
