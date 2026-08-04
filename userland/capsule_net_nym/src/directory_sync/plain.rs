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

use alloc::vec::Vec;

use super::http;
use super::source::DirectorySource;

/// Fetch a small document from a node over plain HTTP.
///
/// No transport security, deliberately. What is asked for here is a node's
/// own published keys, which are public, and every answer is checked against
/// the identity the directory already gave us before it is used. A tampered
/// answer therefore fails that check rather than being believed, and the cost
/// of TLS buys nothing against an adversary who could already tamper.
pub fn fetch_plain(tcp_port: u32, ip: [u8; 4], port: u16, path: &str) -> Result<Vec<u8>, u16> {
    // Nodes answer by address here, so the host header is the address itself.
    let mut host = Vec::with_capacity(16);
    for (index, octet) in ip.iter().enumerate() {
        if index > 0 {
            host.push(b'.');
        }
        write_decimal(&mut host, *octet);
    }
    let source = DirectorySource { ip, port, host, path: path.as_bytes().to_vec() };
    http::fetch(tcp_port, &source)
}

fn write_decimal(out: &mut Vec<u8>, value: u8) {
    let mut buf = [0u8; 3];
    let mut at = buf.len();
    let mut n = value;
    loop {
        at -= 1;
        buf[at] = b'0' + (n % 10);
        n /= 10;
        if n == 0 {
            break;
        }
    }
    out.extend_from_slice(&buf[at..]);
}
