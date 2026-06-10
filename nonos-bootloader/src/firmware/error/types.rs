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

use crate::firmware::types::FirmwareType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
    Fatal,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Loading,
    Validation,
    Security,
    Compatibility,
    Hardware,
    Memory,
    Network,
    Storage,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct ErrorContext {
    pub file_line: u32,
    pub function_id: u16,
    pub thread_id: u8,
    pub additional_data: u64,
}
#[derive(Debug, Clone)]
pub struct FirmwareError {
    pub category: ErrorCategory,
    pub severity: ErrorSeverity,
    pub code: u32,
    pub firmware_type: FirmwareType,
    pub message: [u8; 128],
    pub context: ErrorContext,
    pub timestamp: u64,
}
impl FirmwareError {
    pub fn new(cat: ErrorCategory, sev: ErrorSeverity, code: u32, ft: FirmwareType) -> Self {
        static mut C: u64 = 0;
        Self {
            category: cat,
            severity: sev,
            code,
            firmware_type: ft,
            message: [0; 128],
            context: ErrorContext::default(),
            timestamp: unsafe {
                C += 1;
                C
            },
        }
    }
    pub fn with_message(mut self, msg: &str) -> Self {
        let b = msg.as_bytes();
        self.message[..core::cmp::min(b.len(), 127)]
            .copy_from_slice(&b[..core::cmp::min(b.len(), 127)]);
        self
    }
    pub fn with_context(mut self, ctx: ErrorContext) -> Self {
        self.context = ctx;
        self
    }
    pub fn is_recoverable(&self) -> bool {
        !matches!(self.severity, ErrorSeverity::Fatal)
    }
    pub fn requires_immediate_action(&self) -> bool {
        matches!(self.severity, ErrorSeverity::Critical | ErrorSeverity::Fatal)
    }
    pub fn get_message(&self) -> &str {
        let e = self.message.iter().position(|&b| b == 0).unwrap_or(128);
        core::str::from_utf8(&self.message[..e]).unwrap_or("invalid utf8")
    }
}
impl core::fmt::Display for FirmwareError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{:?}] {:?}: {} (code: {}, firmware: {:?})",
            self.severity,
            self.category,
            self.get_message(),
            self.code,
            self.firmware_type
        )
    }
}
