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

use crate::browser::fetch::keep::{KeptConn, MAX_KEEP_BYTES, MAX_KEEP_USES};
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::http;
use crate::browser::state::State;

// Hold a finished image fetch's connection open for the next same-host image
// when the response framing is intact and the server did not ask to close.
// Returns false when the connection must be closed instead; the caller then
// closes it, so a refused stash is never a leaked socket.
pub(super) fn stash(state: &mut State, job: &mut Fetch, raw: Option<&[u8]>) -> bool {
    if state.keep.is_some() || job.phase != Phase::Decrypt || job.error.is_some() {
        return false;
    }
    let Some(raw) = raw else { return false };
    let Some(frame) = http::response::frame_len(raw) else { return false };
    if wants_close(raw) {
        return false;
    }
    if job.buf.len() > MAX_KEEP_BYTES || job.keep_uses >= MAX_KEEP_USES {
        return false;
    }
    let Some(tls) = job.tls.take() else { return false };
    if tls.server_app.is_none() {
        job.tls = Some(tls);
        return false;
    }
    state.keep = Some(KeptConn {
        host: job.url.host.clone(),
        port: job.url.port,
        handle: job.handle,
        tls,
        buf: core::mem::take(&mut job.buf),
        consumed: job.rx_consumed + frame,
        tx_seq: job.tx_seq + 1,
        used: job.keep_uses + 1,
    });
    true
}

// True when the response headers ask for the connection to be closed.
fn wants_close(raw: &[u8]) -> bool {
    let Some(sep) = raw.windows(4).position(|w| w == b"\r\n\r\n") else { return true };
    let Ok(head) = core::str::from_utf8(&raw[..sep]) else { return true };
    head.lines().skip(1).any(|l| {
        let lower = l.to_ascii_lowercase();
        lower.starts_with("connection:") && lower.contains("close")
    })
}
