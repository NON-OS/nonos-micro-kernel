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

use crate::firmware::detection::version::FirmwareVersion;

#[derive(Debug, Clone)]
pub struct FirmwareMetadata {
    pub version: FirmwareVersion,
    pub vendor: [u8; 32],
    pub description: [u8; 64],
    pub checksum: [u8; 32],
    pub size: u32,
    pub features: u32,
    pub compatibility_flags: u16,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataField {
    Version,
    Vendor,
    Description,
    Checksum,
    Size,
    Features,
    Compatibility,
}
impl Default for FirmwareMetadata {
    fn default() -> Self {
        Self {
            version: FirmwareVersion::default(),
            vendor: [0; 32],
            description: [0; 64],
            checksum: [0; 32],
            size: 0,
            features: 0,
            compatibility_flags: 0,
        }
    }
}

pub fn extract_metadata(data: &[u8]) -> Option<FirmwareMetadata> {
    if data.len() < 128 {
        return None;
    }
    let mut m = FirmwareMetadata::default();
    if let Some(off) = data.windows(4).position(|w| w == b"FWMD") {
        parse_at_offset(data, off, &mut m);
    } else {
        m.size = data.len() as u32;
        m.vendor[..4].copy_from_slice(b"UNKN");
        if data.len() >= 2 {
            m.version.major = data[0];
            m.version.minor = data[1];
        }
    }
    Some(m)
}

pub fn validate_metadata(m: &FirmwareMetadata) -> bool {
    m.size > 0
        && m.size <= 64 * 1024 * 1024
        && !m.vendor.iter().all(|&b| b == 0)
        && !m.checksum.iter().all(|&b| b == 0)
        && (m.version.major > 0 || m.version.minor > 0)
}

fn parse_at_offset(data: &[u8], off: usize, m: &mut FirmwareMetadata) {
    if off + 128 > data.len() {
        return;
    }
    let d = &data[off + 4..off + 128];
    m.version.major = d[0];
    m.version.minor = d[1];
    m.version.patch = u16::from_le_bytes([d[2], d[3]]);
    m.vendor[..32].copy_from_slice(&d[4..36]);
    m.checksum[..32].copy_from_slice(&d[36..68]);
    m.size = u32::from_le_bytes([d[68], d[69], d[70], d[71]]);
    m.features = u32::from_le_bytes([d[72], d[73], d[74], d[75]]);
}
