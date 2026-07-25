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

extern crate alloc;
use alloc::boxed::Box;

use nonos_libc::{heap_init, mk_exit, mk_yield};

use crate::audio_client::AudioClient;
use crate::decode::WavDecoder;
use crate::loader;
use crate::mark::{mark, mark_frames};
use crate::transport::{State, Transport};

const ASSET: &[u8] = b"/audio/boot_tone.wav";
const MAX_PUMPS: u64 = 2_000_000;

pub fn run() -> ! {
    if heap_init().is_err() {
        fail("[PLAYER] heap-fail\n");
    }
    let client = AudioClient::connect().unwrap_or_else(|_| fail("[PLAYER] connect-fail\n"));
    let bytes = loader::load(ASSET).unwrap_or_else(|_| fail("[PLAYER] load-fail\n"));
    let dec = WavDecoder::new(bytes).unwrap_or_else(|_| fail("[PLAYER] decode-fail\n"));
    let mut tp = Transport::new(Box::new(client));
    if tp.open(Box::new(dec)).is_err() {
        fail("[PLAYER] open-fail\n");
    }
    tp.play();
    let mut spins = 0u64;
    while tp.state() != State::Stopped && spins < MAX_PUMPS {
        tp.pump();
        let _ = mk_yield();
        spins += 1;
    }
    if tp.state() == State::Stopped {
        mark_frames(tp.pos_frames());
    } else {
        mark("[PLAYER] timeout\n");
    }
    mk_exit(0);
}

fn fail(m: &str) -> ! {
    mark(m);
    mk_exit(1);
}
