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

use nonos_libc::mk_debug;

/// Say which step of opening a tunnel refused. The client only learns the
/// connect was rejected; no session, no exit and a failed send look identical
/// from there.
pub fn open_failed(step: &[u8], code: u16) {
    let mut line = [0u8; 64];
    let mut n = 0;
    for &b in b"[SOCKS5] open refused: " {
        line[n] = b;
        n += 1;
    }
    for &b in step {
        if n < line.len() - 10 {
            line[n] = b;
            n += 1;
        }
    }
    if code != 0 {
        line[n] = b' ';
        n += 1;
        n += write_u16(&mut line[n..], code);
    }
    line[n] = b'\n';
    mk_debug(line.as_ptr(), n + 1);
}

fn write_u16(out: &mut [u8], mut v: u16) -> usize {
    let mut digits = [0u8; 5];
    let mut k = 0;
    loop {
        digits[k] = b'0' + (v % 10) as u8;
        v /= 10;
        k += 1;
        if v == 0 {
            break;
        }
    }
    for i in 0..k {
        out[i] = digits[k - 1 - i];
    }
    k
}

/// Say what a tunnel did, not only what refused it.
///
/// A connection that opens and then goes quiet looks the same from the client
/// as one that was never opened, so the steps that succeed are worth as much
/// in a log as the ones that fail.
pub fn step(what: &[u8], value: u64) {
    let mut line = [0u8; 80];
    let mut n = 0;
    for &b in b"[SOCKS5] " {
        line[n] = b;
        n += 1;
    }
    for &b in what {
        if n < line.len() - 24 {
            line[n] = b;
            n += 1;
        }
    }
    line[n] = b' ';
    n += 1;
    n += write_u64(&mut line[n..], value);
    line[n] = b'\n';
    mk_debug(line.as_ptr(), n + 1);
}

fn write_u64(out: &mut [u8], mut v: u64) -> usize {
    let mut digits = [0u8; 20];
    let mut k = 0;
    loop {
        digits[k] = b'0' + (v % 10) as u8;
        v /= 10;
        k += 1;
        if v == 0 {
            break;
        }
    }
    for i in 0..k {
        out[i] = digits[k - 1 - i];
    }
    k
}

/// Say where a tunnel was asked to connect, as the exit is asked for it.
///
/// The request the exit receives is an ASCII host and port, and everything
/// upstream of it is guesswork without knowing what that string actually
/// said. A destination that is not what the client typed points at this side
/// of the proxy rather than at the network.
pub fn destination(dest: &crate::conn::Dest) {
    let mut host = [0u8; 288];
    let Some(len) = crate::tunnel::write_hostport(dest, &mut host) else {
        return say(b"[SOCKS5] connect: destination will not render\n");
    };
    let mut line = [0u8; 320];
    let mut n = 0;
    for &b in b"[SOCKS5] connect " {
        line[n] = b;
        n += 1;
    }
    for &b in &host[..len] {
        line[n] = b;
        n += 1;
    }
    line[n] = b'\n';
    say(&line[..n + 1]);
}

/// Report what the exit said back, so a reply that arrives but does not parse
/// is told apart from one that never came.
pub fn reply_bytes(count: usize) {
    let mut line = [0u8; 64];
    let mut n = 0;
    for &b in b"[SOCKS5] exit answered bytes " {
        line[n] = b;
        n += 1;
    }
    n += write_u16(&mut line[n..], count.min(u16::MAX as usize) as u16);
    line[n] = b'\n';
    say(&line[..n + 1]);
}

fn say(line: &[u8]) {
    mk_debug(line.as_ptr(), line.len());
}

/// Report what kind of answer the exit sent.
///
/// Length alone does not say whether a short message is stream data or the
/// exit reporting that it could not reach the host, and those need opposite
/// fixes. The flag distinguishes them, so it is worth naming.
pub fn reply_kind(msg: &[u8]) {
    let text: &[u8] = match msg.get(..2) {
        Some([3, 1]) => b"[SOCKS5] exit sent stream data\n",
        Some([3, 2]) => b"[SOCKS5] exit could not reach the host\n",
        Some([3, other]) => {
            let mut line = *b"[SOCKS5] exit sent an unknown response 000\n";
            let n = line.len();
            line[n - 4] = b'0' + (other / 100) % 10;
            line[n - 3] = b'0' + (other / 10) % 10;
            line[n - 2] = b'0' + other % 10;
            return say(&line);
        }
        _ => b"[SOCKS5] exit sent something this does not speak\n",
    };
    say(text);
}
