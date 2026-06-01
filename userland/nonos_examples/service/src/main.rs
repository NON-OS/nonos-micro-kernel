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

#![no_std]
#![no_main]

use nonos_runtime::prelude::*;

const CAPS: u64 =
    cap::CAP_CORE_EXEC | cap::CAP_IPC | cap::CAP_MEMORY | cap::CAP_REGISTER_SERVICE;
const PORT: u32 = 5000;

fn main() {
    if !service::register(b"example.echo", PORT) {
        exit(1);
    }
    let mut buf = [0u8; 256];
    loop {
        let mut sender: u32 = 0;
        let n = ipc::recv_from(PORT as u64, &mut buf, 0, &mut sender);
        if n > 0 {
            let _ = ipc::reply(sender, &buf[..n as usize]);
        } else {
            yield_now();
        }
    }
}

nonos_main!(CAPS, main);
