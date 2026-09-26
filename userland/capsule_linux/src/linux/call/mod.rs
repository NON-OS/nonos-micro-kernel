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

//! One file per family of Linux calls; declarations and re-exports only.

mod console;
mod ctl;
mod cwd;
mod futex;
mod ident;
mod io;
mod io_socket;
mod life;
mod limits;
mod limits_table;
mod mem;
mod pipe;
mod pipe_dup;
mod pipe_end;
mod pipe_io;
mod pipe_read;
mod session;
mod signal;
mod signal_send;
mod sleep;
mod spawn;
mod thread;
mod timeops;
mod umask;
mod uname;
mod vector;
mod vector_read;

pub use ctl::{fcntl, ioctl};
pub use cwd::{chdir, fchdir, getcwd};
pub use futex::futex;
pub use ident::{getppid, setuid};
pub use io::{close, read, write};
pub use life::{exit, exit_thread};
pub use limits::{getrlimit, prlimit64};
pub use mem::{brk, mmap, mprotect, munmap, MapReq};
pub use pipe::pipe2;
pub use pipe_dup::{dup, dup2};
pub use pipe_io::write as pipe_write;
pub use pipe_read::read as pipe_read;
pub use session::{getpgid, getsid, setpgid, setsid};
pub use signal::{rt_sigaction, rt_sigprocmask, sigaltstack};
pub use signal_send::kill;
pub use sleep::nanosleep;
pub use spawn::{clone, execve, fork, wait4};
pub use thread::{arch_prctl, clock_gettime, getrandom};
pub use timeops::{gettimeofday, time};
pub use umask::{umask, DEFAULT_UMASK};
pub use uname::uname;
pub use vector::writev;
pub use vector_read::readv;
