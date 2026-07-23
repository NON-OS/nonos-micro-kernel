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
use crate::mixer::Mixer;
use crate::sink::Sink;

const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_0010;

pub fn handle(msg: &[u8], mixer: &mut Mixer, sink: &Sink, tx: &mut [u8]) {
    let req = match proto::decode(msg) {
        Some(r) => r,
        None => return,
    };
    let status = route(&req, msg, mixer, sink);
    let n = proto::encode_reply(&req, status, tx);
    if n != 0 {
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), n);
    }
}

fn route(req: &Request, msg: &[u8], mixer: &mut Mixer, sink: &Sink) -> i32 {
    match req.op {
        proto::OP_PLAY_TONE => ops::play_tone(req, msg, mixer, sink),
        proto::OP_PLAY_PCM => ops::play_pcm(req, msg, mixer, sink),
        proto::OP_STOP => {
            mixer.clear();
            proto::E_OK
        }
        _ => proto::E_INVAL,
    }
}
