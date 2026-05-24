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
use crate::state::Scanout;
#[derive(Clone, Copy)]
pub struct Geometry {
    pub stride: u32,
    pub byte_len: u64,
}
pub fn derive(scanout: Scanout) -> Result<Option<Geometry>, &'static str> {
    if !scanout.enabled || scanout.width == 0 || scanout.height == 0 {
        return Ok(None);
    }
    let stride = scanout.width.checked_mul(4).ok_or("virtio-gpu: stride overflow")?;
    let byte_len =
        (stride as u64).checked_mul(scanout.height as u64).ok_or("virtio-gpu: surface overflow")?;
    if byte_len > u32::MAX as u64 {
        return Err("virtio-gpu: surface too large");
    }
    Ok(Some(Geometry { stride, byte_len }))
}
