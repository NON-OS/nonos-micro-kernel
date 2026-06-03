// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Smoke-only dispatch tracing. Prints a bounded number of
//! `[SC <kind>]` lines on the boot serial. Compiled in only under
//! `nonos-user-entry-proof`; production builds carry no syscall trace.

use core::sync::atomic::{AtomicU32, Ordering};

use super::numbers::*;

static SHOWN: AtomicU32 = AtomicU32::new(0);
const CAP: u32 = 32;

pub(super) fn enter(nr: u64, a0: u64) {
    if SHOWN.load(Ordering::Relaxed) >= CAP {
        return;
    }
    crate::sys::serial::print(b"[SC ");
    crate::sys::serial::print(kind(nr));
    crate::sys::serial::print(b"] pid=");
    crate::arch::x86_64::diag::print_hex_u64(crate::process::current_pid().unwrap_or(0) as u64);
    crate::sys::serial::print(b" a0=");
    crate::arch::x86_64::diag::print_hex_u64(a0);
    crate::sys::serial::println(b"");
}

pub(super) fn exit(nr: u64, r: i64) {
    if SHOWN.fetch_add(1, Ordering::Relaxed) >= CAP {
        return;
    }
    crate::sys::serial::print(b"[SC ");
    crate::sys::serial::print(kind(nr));
    crate::sys::serial::print(b"] -> ");
    crate::arch::x86_64::diag::print_hex_u64(r as u64);
    crate::sys::serial::println(b"");
}

pub(super) fn unknown(nr: u64) {
    if SHOWN.load(Ordering::Relaxed) >= CAP {
        return;
    }
    crate::sys::serial::print(b"[SC unknown] nr=");
    crate::arch::x86_64::diag::print_hex_u64(nr);
    crate::sys::serial::println(b"");
}

fn kind(nr: u64) -> &'static [u8] {
    match nr {
        SYS_IPC_SEND => b"MkIpcSend",
        SYS_IPC_RECV => b"MkIpcRecv",
        SYS_IPC_CALL => b"MkIpcCall",
        SYS_IPC_REPLY => b"MkIpcReply",
        SYS_MMAP => b"MkMmap",
        SYS_MUNMAP => b"MkMunmap",
        SYS_EXIT => b"MkExit",
        SYS_MK_DEBUG => b"MkDebug",
        SYS_YIELD => b"MkYield",
        SYS_TIME_MILLIS => b"MkTimeMillis",
        SYS_TIME_RTC => b"MkTimeRtc",
        SYS_SPAWN => b"MkSpawn",
        _ => b"Mk?",
    }
}
