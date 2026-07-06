// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Transparent, post-quantum STARK verification primitives. Hash-based and
//! curve-free, so verification relies only on the strength of the hash. Built
//! bottom up: the Goldilocks field, then a Merkle commitment over it.

pub mod field;
pub mod merkle;
