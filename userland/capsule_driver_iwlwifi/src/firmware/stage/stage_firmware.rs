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

use super::count_section::count_section;
use super::stage_section::stage_section;
use super::state::FirmwareStageState;
use crate::firmware::tlv::{
    le32, parse_header, TLV_HEADER_LEN, TLV_PAGING, TLV_SEC_INIT, TLV_SEC_RT,
};

pub fn stage_firmware(data: &[u8], dma_user_va: u64, dma_len: u64) -> Option<FirmwareStageState> {
    let h = parse_header(data)?;
    let mut state = FirmwareStageState {
        major: h.major,
        minor: h.minor,
        api: h.api,
        build: h.build,
        ..FirmwareStageState::empty()
    };
    let mut off = TLV_HEADER_LEN;
    let mut dst = 0usize;
    while off + 8 <= data.len() {
        let ty = le32(data, off)?;
        let len = le32(data, off + 4)? as usize;
        off += 8;
        if off + len > data.len() {
            return None;
        }
        if matches!(ty, TLV_SEC_RT | TLV_SEC_INIT | TLV_PAGING) && len >= 4 {
            stage_section(data, off, len, dma_user_va, dma_len as usize, &mut dst)?;
            count_section(ty, &mut state);
        }
        off += (len + 3) & !3;
    }
    state.staged_bytes = dst as u32;
    Some(state)
}
