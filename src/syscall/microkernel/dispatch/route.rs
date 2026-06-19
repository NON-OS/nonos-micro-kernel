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

use super::args::Args;
use super::{capability, debug, device, dma, ipc, irq, mmio, pio, process};

pub fn dispatch_microkernel_syscall(
    nr: u64,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> i64 {
    let args = Args::new(a0, a1, a2, a3, a4, a5);
    route(nr, args)
}

fn route(nr: u64, args: Args) -> i64 {
    if let Some(result) = ipc::handle(nr, args) {
        return result;
    }
    if let Some(result) = process::handle(nr, args) {
        return result;
    }
    if let Some(result) = capability::handle(nr, args) {
        return result;
    }
    if let Some(result) = device::handle(nr, args) {
        return result;
    }
    if let Some(result) = irq::handle(nr, args) {
        return result;
    }
    route_tail(nr, args)
}

fn route_tail(nr: u64, args: Args) -> i64 {
    if let Some(result) = mmio::handle(nr, args) {
        return result;
    }
    if let Some(result) = dma::handle(nr, args) {
        return result;
    }
    if let Some(result) = pio::handle(nr, args) {
        return result;
    }
    if let Some(result) = debug::handle(nr, args) {
        return result;
    }
    -1
}
