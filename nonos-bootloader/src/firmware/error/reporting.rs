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

#![allow(static_mut_refs)]

use super::types::{ErrorSeverity, FirmwareError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportingLevel {
    None,
    Minimal,
    Standard,
    Verbose,
    Debug,
}
#[derive(Debug, Clone)]
pub struct ErrorReport {
    error: FirmwareError,
    report_level: ReportingLevel,
    formatted_message: [u8; 512],
    report_id: u32,
}
static mut ERROR_REPORTS: [Option<ErrorReport>; 64] = [const { None }; 64];
static mut REPORT_COUNT: usize = 0;

pub fn report_error(error: FirmwareError, level: ReportingLevel) -> u32 {
    static mut NEXT_ID: u32 = 1;
    let report_id = unsafe {
        NEXT_ID += 1;
        NEXT_ID - 1
    };
    let formatted_message = format_error_message(&error, level);
    let report = ErrorReport { error, report_level: level, formatted_message, report_id };
    unsafe {
        if REPORT_COUNT < ERROR_REPORTS.len() {
            ERROR_REPORTS[REPORT_COUNT] = Some(report);
            REPORT_COUNT += 1;
        }
    }
    report_id
}

pub fn format_error_message(error: &FirmwareError, level: ReportingLevel) -> [u8; 512] {
    let mut msg = [0u8; 512];
    let s = match level {
        ReportingLevel::None => return msg,
        ReportingLevel::Minimal => alloc::format!("Error {}: {}", error.code, error.get_message()),
        ReportingLevel::Standard => alloc::format!(
            "[{:?}] Error {}: {} (firmware: {:?})",
            error.severity,
            error.code,
            error.get_message(),
            error.firmware_type
        ),
        ReportingLevel::Verbose => alloc::format!(
            "[{:?}] {:?} Error {}: {} (firmware: {:?}, time: {})",
            error.severity,
            error.category,
            error.code,
            error.get_message(),
            error.firmware_type,
            error.timestamp
        ),
        ReportingLevel::Debug => alloc::format!(
            "[{:?}] {:?} Error {}: {} (fw: {:?}, t: {}, ln: {}, fn: {})",
            error.severity,
            error.category,
            error.code,
            error.get_message(),
            error.firmware_type,
            error.timestamp,
            error.context.file_line,
            error.context.function_id
        ),
    };
    let b = s.as_bytes();
    let len = core::cmp::min(b.len(), 511);
    msg[..len].copy_from_slice(&b[..len]);
    msg
}

impl ErrorReport {
    pub fn get_formatted_message(&self) -> &str {
        let e = self.formatted_message.iter().position(|&b| b == 0).unwrap_or(512);
        core::str::from_utf8(&self.formatted_message[..e]).unwrap_or("invalid")
    }
    pub fn get_severity(&self) -> ErrorSeverity {
        self.error.severity
    }
    pub fn get_id(&self) -> u32 {
        self.report_id
    }
    pub fn get_report_level(&self) -> ReportingLevel {
        self.report_level
    }
}
