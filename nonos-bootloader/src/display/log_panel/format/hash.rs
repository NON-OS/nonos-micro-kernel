// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::hex::HEX;
use crate::display::log_panel::api::log;
use crate::display::log_panel::helpers::copy_prefix;
use crate::display::log_panel::types::LogLevel;

pub fn log_hash(prefix: &[u8], hash: &[u8]) {
    let mut buf = [0u8; 58];
    let mut p = copy_prefix(&mut buf, prefix);
    for &b in hash.iter().take(16) {
        if p + 2 > buf.len() {
            break;
        }
        buf[p] = HEX[(b >> 4) as usize];
        buf[p + 1] = HEX[(b & 0xF) as usize];
        p += 2;
    }
    log(LogLevel::Ok, &buf[..p]);
    if hash.len() > 16 {
        let mut b2 = [0u8; 58];
        b2[0..6].copy_from_slice(b"      ");
        let mut p2 = 6;
        for &b in hash.iter().skip(16) {
            if p2 + 2 > b2.len() {
                break;
            }
            b2[p2] = HEX[(b >> 4) as usize];
            b2[p2 + 1] = HEX[(b & 0xF) as usize];
            p2 += 2;
        }
        log(LogLevel::Info, &b2[..p2]);
    }
}

pub fn log_hash_full(label: &[u8], hash: &[u8]) {
    log(LogLevel::Ok, label);
    let mut buf = [0u8; 72];
    buf[0..4].copy_from_slice(b"  0x");
    let mut p = 4;
    for &b in hash {
        if p + 2 <= buf.len() {
            buf[p] = HEX[(b >> 4) as usize];
            buf[p + 1] = HEX[(b & 0xF) as usize];
            p += 2;
        }
    }
    log(LogLevel::Info, &buf[..p]);
}
