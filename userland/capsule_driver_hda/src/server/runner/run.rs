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

use alloc::vec;

use nonos_libc::mk_ipc_recv;

use super::max_tx_body::max_tx_body;
use super::poll_irq::poll_irq;
use crate::protocol::{
    decode_request, E_INVAL, HDR_LEN, MAX_PCM_CHUNK, OP_CODEC_LIST, OP_CODEC_MASK,
    OP_CONTROLLER_INFO, OP_HEALTHCHECK, OP_PLAY_TONE, OP_STREAM_LAYOUT, OP_STREAM_START,
    OP_WRITE_PCM, RESP_HDR_LEN, SERVICE_NAME,
};
use crate::server::{error, handlers};
use crate::setup::Driver;

const TX_LEN: usize = RESP_HDR_LEN + 4 + max_tx_body();
const RX_LEN: usize = HDR_LEN + MAX_PCM_CHUNK;

pub fn run(driver: Driver) -> ! {
    let mut rx = vec![0u8; RX_LEN];
    let mut tx = vec![0u8; TX_LEN];
    let mut last_irq_seq = 0u64;
    let mut played = false;
    let mut running = false;
    let mut pcm_q = crate::audio::PcmQueue::new();
    let _service_name = SERVICE_NAME;
    loop {
        poll_irq(&driver, &mut last_irq_seq, &mut played);
        let n = mk_ipc_recv(0, rx.as_mut_ptr(), RX_LEN, 0);
        if n <= 0 {
            continue;
        }
        let req = match decode_request(&rx[..n as usize]) {
            Some(r) => r,
            None => continue,
        };
        if req.payload_len != 0 && req.op != OP_WRITE_PCM {
            error::reply_with_status(&mut tx, &req, E_INVAL);
            continue;
        }
        match req.op {
            OP_HEALTHCHECK => handlers::health::handle(&req, &mut tx),
            OP_CONTROLLER_INFO => handlers::controller_info::handle(&driver, &req, &mut tx),
            OP_CODEC_MASK => handlers::codec_mask::handle(&driver, &req, &mut tx),
            OP_STREAM_LAYOUT => handlers::stream_layout::handle(&driver, &req, &mut tx),
            OP_CODEC_LIST => handlers::codec_list::handle(&driver, &req, &mut tx),
            OP_PLAY_TONE => handlers::play_tone::handle(&driver, &req, &mut tx, &mut played),
            OP_STREAM_START => {
                handlers::stream::handle_stream_start(&driver, &req, &mut tx, &mut running)
            }
            OP_WRITE_PCM => {
                handlers::write_pcm::handle(&mut pcm_q, &req, &rx[..n as usize], &mut tx)
            }
            _ => error::reply_with_status(&mut tx, &req, E_INVAL),
        }
    }
}
