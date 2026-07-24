// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! #402 IPC reply correlation (recv_reply_correlated in
//! src/syscall/microkernel/ipc/recv.rs). Deliver only the message whose
//! correlation equals the nonzero call token; sys_ipc_send hardcodes 0, so a
//! forged frame can never match.

/// Scan the inbox, returning the first message whose correlation matches the
/// call token and dropping the rest.
pub fn first_match(call_tok: u64, inbox: &[u64]) -> Option<u64> {
    inbox.iter().copied().find(|&corr| corr == call_tok)
}
