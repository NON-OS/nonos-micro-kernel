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

/// Normalize a GOP scanline stride to bytes for the handoff contract.
///
/// The spec says PixelsPerScanLine holds pixels, but firmware in the wild
/// disagrees: OVMF builds have been seen reporting bytes in that field,
/// while real laptop firmware reports pixels. The two ranges cannot
/// overlap for 32-bit modes: a byte stride is always at least width * 4,
/// and pixel padding never reaches four times the width. Below width the
/// value is garbage either way.
pub fn stride_to_bytes(stride: u32, width: u32) -> Option<u32> {
    let min_bytes = width.checked_mul(4)?;
    if stride >= min_bytes {
        return Some(stride);
    }
    if stride >= width {
        return stride.checked_mul(4);
    }
    None
}
