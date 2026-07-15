// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

/// The standard INQUIRY data length we request and decode (SPC "standard
/// inquiry data" through the product-revision field).
pub const INQUIRY_DATA_LEN: usize = 36;

/// The identity a device reports in its standard INQUIRY data. `peripheral_type`
/// 0x00 is a direct-access block device (a disk); `removable` distinguishes a
/// flash drive from a fixed disk.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InquiryData {
    pub peripheral_type: u8,
    pub removable: bool,
    pub version: u8,
    pub vendor: [u8; 8],
    pub product: [u8; 16],
    pub revision: [u8; 4],
}

/// Decode the standard INQUIRY data. Returns None if the device returned fewer
/// bytes than the fixed fields require.
pub fn parse_inquiry(raw: &[u8]) -> Option<InquiryData> {
    if raw.len() < INQUIRY_DATA_LEN {
        return None;
    }
    let mut vendor = [0u8; 8];
    vendor.copy_from_slice(&raw[8..16]);
    let mut product = [0u8; 16];
    product.copy_from_slice(&raw[16..32]);
    let mut revision = [0u8; 4];
    revision.copy_from_slice(&raw[32..36]);
    Some(InquiryData {
        peripheral_type: raw[0] & 0x1f,
        removable: raw[1] & 0x80 != 0,
        version: raw[2],
        vendor,
        product,
        revision,
    })
}
