// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Runnable proof-of-concept for the trusted-path audit fixes (PR #405). Each
//! module mirrors the exact decision the kernel change makes; the tests in the
//! tests directory assert the bug is closed, next to the Lean proofs.

pub mod frame_alloc;
pub mod framebuffer;
pub mod input_authority;
pub mod register_auth;
pub mod reply_correlation;
