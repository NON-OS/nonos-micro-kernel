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

use super::read;
use super::types::{Frame, FrameKind};

pub fn next(buf: &[u8], out: &mut [u8], ctrl: &mut [u8; 125]) -> Result<Option<Frame>, u16> {
    if buf.len() < 2 {
        return Ok(None);
    }
    if buf[0] & 0x80 == 0 {
        return Err(read::E_BAD_FRAME);
    }
    let opcode = buf[0] & 0x0f;
    let masked = buf[1] & 0x80 != 0;
    let Some((len, off)) = read::frame_len(buf, (buf[1] & 0x7f) as usize)? else {
        return Ok(None);
    };
    let mask_off = if masked { 4 } else { 0 };
    let start = off + mask_off;
    if buf.len() < start + len {
        return Ok(None);
    }
    let consumed = start + len;
    match opcode {
        // Nym's control frames are text and its mix packets binary; both carry
        // a payload identically, so only the reported kind differs.
        1 => read::copy_payload(buf, out, masked, len, off)
            .map(|n| Some(frame(FrameKind::Text, n, consumed))),
        2 => read::copy_payload(buf, out, masked, len, off)
            .map(|n| Some(frame(FrameKind::Binary, n, consumed))),
        8 => Ok(Some(frame(FrameKind::Close, len, consumed))),
        9 => control(buf, ctrl, masked, len, off, FrameKind::Ping, consumed),
        10 => Ok(Some(frame(FrameKind::Pong, len, consumed))),
        _ => Err(read::E_BAD_FRAME),
    }
}

fn control(
    buf: &[u8],
    ctrl: &mut [u8; 125],
    masked: bool,
    len: usize,
    off: usize,
    kind: FrameKind,
    consumed: usize,
) -> Result<Option<Frame>, u16> {
    if len > ctrl.len() {
        return Err(read::E_BAD_FRAME);
    }
    read::copy_payload(buf, ctrl, masked, len, off)?;
    Ok(Some(frame(kind, len, consumed)))
}

fn frame(kind: FrameKind, len: usize, consumed: usize) -> Frame {
    Frame { kind, len, consumed }
}
