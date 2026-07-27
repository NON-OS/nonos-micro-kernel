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
use crate::decode::open as decode_open;
use crate::loader;
use crate::mark::{mark, mark_frames, mark_mp3_frames};
use crate::transport::{State, Transport};

const ASSET: &[u8] = b"/audio/boot_tone.wav";
const MP3_ASSET: &[u8] = b"/audio/boot_tone.mp3";
const MAX_PUMPS: u64 = 2_000_000;

pub fn run() -> ! {
    if heap_init().is_err() {
        fail("[PLAYER] heap-fail\n");
    }
    let client = AudioClient::connect().unwrap_or_else(|_| fail("[PLAYER] connect-fail\n"));
    let bytes = loader::load(ASSET).unwrap_or_else(|_| fail("[PLAYER] load-fail\n"));
    let dec = decode_open(bytes).unwrap_or_else(|_| fail("[PLAYER] decode-fail\n"));
    let mut tp = Transport::new(Box::new(client));
    if tp.open(dec).is_err() {
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
    let mp3 = loader::load(MP3_ASSET).unwrap_or_else(|_| fail("[PLAYER] mp3-load-fail\n"));
    let mut mdec = decode_open(mp3).unwrap_or_else(|_| fail("[PLAYER] mp3-decode-fail\n"));
    let ch = mdec.info().channels.max(1) as u64;
    let mut buf = [0i16; 4096];
    let mut mp3_frames = 0u64;
    loop {
        let n = mdec.next(&mut buf);
        if n == 0 {
            break;
        }
        mp3_frames += (n as u64) / ch;
    }
    mark_mp3_frames(mp3_frames);
    mk_exit(0);
}

fn fail(m: &str) -> ! {
    mark(m);
    mk_exit(1);
}
