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

use super::mmio::{
    read_mmio_u32, read_mmio_u8, TPM_BASE_ADDR, TPM_DID_VID_OFFSET, TPM_INTF_CAP_OFFSET,
    TPM_RID_OFFSET,
};
use super::types::TpmCapabilities;

pub fn detect_tpm_capabilities() -> TpmCapabilities {
    let mut caps = TpmCapabilities::default();
    if !probe_presence() {
        return caps;
    }
    caps.present = true;
    read_identity(&mut caps);
    read_interface(&mut caps);
    caps
}

fn probe_presence() -> bool {
    let did_vid = unsafe { read_mmio_u32(TPM_BASE_ADDR + TPM_DID_VID_OFFSET) };
    did_vid != 0xFFFF_FFFF && did_vid != 0
}

fn read_identity(c: &mut TpmCapabilities) {
    let did_vid = unsafe { read_mmio_u32(TPM_BASE_ADDR + TPM_DID_VID_OFFSET) };
    let rid = unsafe { read_mmio_u8(TPM_BASE_ADDR + TPM_RID_OFFSET) };
    c.vendor_id = (did_vid & 0xFFFF) as u16;
    c.device_id = ((did_vid >> 16) & 0xFFFF) as u16;
    c.revision_id = rid;
    c.version_2_0 = c.vendor_id != 0 && c.device_id != 0;
}

fn read_interface(c: &mut TpmCapabilities) {
    let intf = unsafe { read_mmio_u32(TPM_BASE_ADDR + TPM_INTF_CAP_OFFSET) };
    c.locality_count = ((intf >> 8) & 0x07) as u8 + 1;
    c.sha256_supported = true;
    c.sha384_supported = (intf & (1 << 4)) != 0;
    c.rsa_supported = true;
    c.ecc_supported = (intf & (1 << 7)) != 0;
}
