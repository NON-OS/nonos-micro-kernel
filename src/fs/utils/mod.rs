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

extern crate alloc;

pub mod classify;
pub mod error;
pub mod helpers;
pub mod matchers;
pub mod patterns;
pub mod scan_basic;
pub mod scan_config;
pub mod scan_stats;
pub mod types;

pub use classify::*;
pub use error::{UtilsError, UtilsResult};
pub use helpers::*;
pub use matchers::*;
pub use scan_basic::*;
pub use scan_config::*;
pub use scan_stats::{count_files_by_sensitivity, get_scan_operation_count, get_scan_statistics};
pub use types::*;

use alloc::{string::String, vec::Vec};

pub fn list_hidden_files(dir_path: &str) -> Vec<String> {
    scan_basic::scan_hidden_files(dir_path)
}

pub fn scan_for_sensitive_files(dir_path: &str) -> Vec<String> {
    scan_basic::scan_sensitive_files(dir_path)
}

pub fn scan_files_with_config(dir_path: &str, config: &ScanConfig) -> UtilsResult<Vec<ScanResult>> {
    scan_config::scan_with_config(dir_path, config)
}

pub fn classify_file(path: &str) -> FileClassification {
    classify::classify_file_by_path(path)
}

#[inline]
pub fn is_sensitive_file(path: &str) -> bool {
    matchers::matches_sensitive_pattern(path)
}

#[inline]
pub fn is_hidden_file(path: &str) -> bool {
    helpers::is_hidden(path)
}
