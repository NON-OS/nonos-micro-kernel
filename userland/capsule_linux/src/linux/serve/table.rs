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

//! Every number a guest can ask for.

use crate::linux::abi::{errno, nr, nr_path as np};
use crate::linux::call;
use crate::linux::guest::Guest;

use super::table_file::file_ops;
use super::table_mem::mem_ops;
use super::table_net::net_ops;
use super::table_proc::proc_ops;

pub fn plain(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> u64 {
    if let Some(v) = file_ops(guest, tid, nr, a) {
        return v;
    }
    if let Some(v) = net_ops(guest, tid, nr, a) {
        return v;
    }
    if let Some(v) = mem_ops(guest, nr, a) {
        return v;
    }
    if let Some(v) = proc_ops(guest, nr, a) {
        return v;
    }
    rest(guest, tid, nr, a)
}

fn rest(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> u64 {
    match nr {
        nr::IOCTL => call::ioctl(guest, a[0], a[1]),
        nr::FCNTL => call::fcntl(guest, a[0], a[1], a[2]),
        nr::UNAME => call::uname(guest, a[0]),
        np::GETRLIMIT => call::getrlimit(guest, a[0], a[1]),
        np::UMASK => call::umask(guest, a[0]),
        np::PRLIMIT64 => call::prlimit64(guest, a[1], a[2], a[3]),
        nr::RT_SIGACTION => call::rt_sigaction(guest, a[0], a[1], a[2]),
        nr::RT_SIGPROCMASK => call::rt_sigprocmask(guest, a[2]),
        nr::SIGALTSTACK => call::sigaltstack(guest, a[1]),
        nr::RSEQ | nr::SET_ROBUST_LIST => errno::ok(0),
        nr::ARCH_PRCTL => call::arch_prctl(guest, tid, a[0], a[1]),
        nr::GETRANDOM => call::getrandom(guest, a[0], a[1], a[2]),
        nr::EXIT | nr::EXIT_GROUP => call::exit(guest, a[0]),
        other => super::unserved::unserved(other),
    }
}
