// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::types::{ErrorCategory, FirmwareError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStrategy {
    Retry,
    Fallback,
    Ignore,
    Reset,
    Abort,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryResult {
    Success,
    PartialSuccess,
    Failed,
    NotApplicable,
}
#[derive(Debug, Clone)]
pub struct ErrorRecovery {
    strategy: RecoveryStrategy,
    max_retries: u8,
    fallback_options: u8,
    recovery_timeout: u32,
}
impl Default for ErrorRecovery {
    fn default() -> Self {
        Self {
            strategy: RecoveryStrategy::Retry,
            max_retries: 3,
            fallback_options: 2,
            recovery_timeout: 5000,
        }
    }
}
impl ErrorRecovery {
    pub fn new(strategy: RecoveryStrategy) -> Self {
        Self { strategy, ..Self::default() }
    }
    pub fn with_retries(mut self, r: u8) -> Self {
        self.max_retries = r;
        self
    }
    pub fn with_timeout(mut self, t: u32) -> Self {
        self.recovery_timeout = t;
        self
    }
    pub fn with_fallback_count(mut self, c: u8) -> Self {
        self.fallback_options = c;
        self
    }
}

pub fn attempt_error_recovery(err: &FirmwareError, rec: &ErrorRecovery) -> RecoveryResult {
    if !err.is_recoverable() {
        return RecoveryResult::NotApplicable;
    }
    match rec.strategy {
        RecoveryStrategy::Retry => retry(err, rec.max_retries),
        RecoveryStrategy::Fallback => fallback(err, rec.fallback_options),
        RecoveryStrategy::Ignore => RecoveryResult::Success,
        RecoveryStrategy::Reset => reset(err),
        RecoveryStrategy::Abort => RecoveryResult::Failed,
    }
}

fn retry(err: &FirmwareError, max: u8) -> RecoveryResult {
    for i in 0..max {
        if matches!(err.category, ErrorCategory::Loading | ErrorCategory::Validation) {
            return if i == 0 { RecoveryResult::Success } else { RecoveryResult::PartialSuccess };
        }
        for _ in 0..((1u32 << i) * 1000) {
            core::hint::spin_loop();
        }
    }
    RecoveryResult::Failed
}

fn fallback(err: &FirmwareError, opts: u8) -> RecoveryResult {
    for o in 0..opts {
        if err.category == ErrorCategory::Loading && o < 2 {
            return if o == 0 { RecoveryResult::Success } else { RecoveryResult::PartialSuccess };
        }
    }
    RecoveryResult::Failed
}

fn reset(err: &FirmwareError) -> RecoveryResult {
    match err.category {
        ErrorCategory::Hardware | ErrorCategory::Memory => RecoveryResult::Success,
        ErrorCategory::Loading => RecoveryResult::PartialSuccess,
        _ => RecoveryResult::NotApplicable,
    }
}
