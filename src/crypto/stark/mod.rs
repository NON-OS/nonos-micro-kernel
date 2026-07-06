// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Transparent, post-quantum STARK verification primitives. Hash-based and
//! curve-free, so verification relies only on the strength of the hash. Built
//! bottom up starting from the Goldilocks field.

pub mod field;
