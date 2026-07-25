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

use nonos_libc::mk_ipc_send;

use super::ops;
use super::proto::{self, Request};
use super::pump::PumpState;
use super::streams::StreamTable;
use crate::mixer::Mixer;
use crate::sink::Sink;

const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_0012;

pub fn handle(
    msg: &[u8],
    mixer: &mut Mixer,
    sink: &Sink,
    table: &mut StreamTable,
    pump: &mut PumpState,
    tx: &mut [u8],
) {
    let req = match proto::decode(msg) {
        Some(r) => r,
        None => return,
    };
    if req.op == proto::OP_STREAM_OPEN {
        let (status, id) = ops::stream_open(&req, msg, table);
        let n = proto::encode_open_reply(&req, status, id, tx);
        if n != 0 {
            let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), n);
        }
        return;
    }
    let status = route(&req, msg, mixer, sink, table, pump);
    let n = proto::encode_reply(&req, status, tx);
    if n != 0 {
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), n);
    }
}

fn route(
    req: &Request,
    msg: &[u8],
    mixer: &mut Mixer,
    sink: &Sink,
    table: &mut StreamTable,
    pump: &mut PumpState,
) -> i32 {
    match req.op {
        proto::OP_PLAY_TONE => ops::play_tone(req, msg, mixer, sink),
        proto::OP_PLAY_PCM => ops::play_pcm(req, msg, mixer, sink),
        proto::OP_STOP => {
            mixer.clear();
            proto::E_OK
        }
        proto::OP_FEED_PCM => ops::stream_feed(req, msg, table, pump, sink),
        proto::OP_PAUSE => ops::stream_pause(req, msg, table),
        proto::OP_CLOSE => ops::stream_close(req, msg, table),
        _ => proto::E_INVAL,
    }
}
