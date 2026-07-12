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

//! Legacy Transmit Frame Descriptor (TFD) fill. A TFD is a 128-byte descriptor:
//! a 4-byte header whose last byte is the transfer-buffer count, then up to 20
//! transfer buffers of `{low 32 bits of the DRAM address, then 4 high address
//! bits and a 12-bit length}`. The firmware reads the TFD at a queue's write
//! pointer to find the command or frame in host DRAM. Byte layout from
//! iwl-fh.h, checked by `iwlwifi_proofs`.

pub const TFD_SIZE: usize = 128;
pub const TFD_MAX_TB_LEN: u16 = 0x0FFF;
const TB_OFFSET: usize = 4;

/// Fill the TFD in `tfd` with a single transfer buffer pointing at device
/// address `addr` for `len` bytes. Returns false if `tfd` is too small or `len`
/// exceeds the 12-bit length field.
pub fn fill_single(tfd: &mut [u8], addr: u64, len: u16) -> bool {
    if tfd.len() < TFD_SIZE || len > TFD_MAX_TB_LEN {
        return false;
    }
    for b in tfd[..TFD_SIZE].iter_mut() {
        *b = 0;
    }
    tfd[3] = 1;
    let lo = (addr & 0xFFFF_FFFF) as u32;
    let hi_n_len = ((addr >> 32) as u16 & 0xF) | ((len & TFD_MAX_TB_LEN) << 4);
    tfd[TB_OFFSET..TB_OFFSET + 4].copy_from_slice(&lo.to_le_bytes());
    tfd[TB_OFFSET + 4..TB_OFFSET + 6].copy_from_slice(&hi_n_len.to_le_bytes());
    true
}
