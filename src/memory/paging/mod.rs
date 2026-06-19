// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate alloc;

pub mod constants;
pub mod error;
pub mod manager;
mod stats;
pub mod tlb;
pub mod types;

pub use constants::*;
pub use error::{PageFaultInfo, PagingError, PagingResult};
pub use manager::*;
pub use types::{PageMapping, PagePermissions, PageSize, PagingStats};

use core::sync::atomic::{AtomicBool, Ordering};

static ASLR_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_aslr_enabled(enabled: bool) {
    ASLR_ENABLED.store(enabled, Ordering::SeqCst);
}
pub fn is_aslr_enabled() -> bool {
    ASLR_ENABLED.load(Ordering::Relaxed)
}
