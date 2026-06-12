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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
    pub build: u16,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionComparison {
    Newer,
    Same,
    Older,
    Incompatible,
}
impl Default for FirmwareVersion {
    fn default() -> Self {
        Self { major: 0, minor: 0, patch: 0, build: 0 }
    }
}
impl core::fmt::Display for FirmwareVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.build > 0 {
            write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, self.build)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}

pub fn parse_firmware_version(s: &str) -> Option<FirmwareVersion> {
    let parts: alloc::vec::Vec<&str> = s.split('.').collect();
    if parts.len() < 3 {
        return None;
    }
    let major = parts[0].parse::<u8>().ok()?;
    let minor = parts[1].parse::<u8>().ok()?;
    let pb: alloc::vec::Vec<&str> = parts[2].split('-').collect();
    let patch = pb[0].parse::<u16>().ok()?;
    let build = if pb.len() > 1 { pb[1].parse::<u16>().unwrap_or(0) } else { 0 };
    Some(FirmwareVersion { major, minor, patch, build })
}

pub fn compare_versions(a: &FirmwareVersion, b: &FirmwareVersion) -> VersionComparison {
    if a.major != b.major {
        return VersionComparison::Incompatible;
    }
    match (a.minor.cmp(&b.minor), a.patch.cmp(&b.patch), a.build.cmp(&b.build)) {
        (core::cmp::Ordering::Greater, _, _) => VersionComparison::Newer,
        (core::cmp::Ordering::Less, _, _) => VersionComparison::Older,
        (_, core::cmp::Ordering::Greater, _) => VersionComparison::Newer,
        (_, core::cmp::Ordering::Less, _) => VersionComparison::Older,
        (_, _, core::cmp::Ordering::Greater) => VersionComparison::Newer,
        (_, _, core::cmp::Ordering::Less) => VersionComparison::Older,
        _ => VersionComparison::Same,
    }
}
