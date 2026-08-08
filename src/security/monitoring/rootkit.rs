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

use alloc::{string::String, vec::Vec};
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

#[derive(Debug, Clone)]
pub struct RootkitScanResult {
    pub timestamp: u64,
    pub suspicious_modules: Vec<String>,
    pub suspicious_files: Vec<String>,
    pub suspicious_syscalls: Vec<u32>,
    pub kernel_modifications: Vec<String>,
    pub alerts: Vec<String>,
    pub score: u8,
}

static LAST_SCAN_RESULT: Mutex<Option<RootkitScanResult>> = Mutex::new(None);
static SCAN_COUNTER: AtomicU64 = AtomicU64::new(0);
static BASELINE_CAPTURED: core::sync::atomic::AtomicBool =
    core::sync::atomic::AtomicBool::new(false);

pub fn init() -> Result<(), &'static str> {
    if BASELINE_CAPTURED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let initial_scan = scan_system();
    if initial_scan.score > 0 {
        crate::log::warn!(
            "[ROOTKIT] Initial scan found {} alert(s) during boot",
            initial_scan.alerts.len()
        );
    }
    crate::log::info!("[ROOTKIT] Monitoring initialized, baseline captured");
    Ok(())
}

pub fn scan_system() -> RootkitScanResult {
    let suspicious_modules = scan_loaded_modules();
    let suspicious_files = scan_filesystem();
    let suspicious_syscalls: Vec<u32> = Vec::new();
    let kernel_modifications = scan_kernel_integrity();

    let mut alerts = Vec::new();

    if !suspicious_modules.is_empty() {
        alerts.push(format!("Suspicious modules: {:?}", suspicious_modules));
    }
    if !suspicious_files.is_empty() {
        alerts.push(format!("Suspicious files: {:?}", suspicious_files));
    }
    if !kernel_modifications.is_empty() {
        alerts.push(format!("Kernel memory modified: {:?}", kernel_modifications));
    }

    let score = if alerts.is_empty() { 0 } else { 20 * alerts.len() as u8 };

    let result = RootkitScanResult {
        timestamp: crate::time::timestamp_millis(),
        suspicious_modules,
        suspicious_files,
        suspicious_syscalls,
        kernel_modifications,
        alerts,
        score,
    };

    {
        let mut lock = LAST_SCAN_RESULT.lock();
        *lock = Some(result.clone());
    }
    SCAN_COUNTER.fetch_add(1, Ordering::Relaxed);
    result
}

pub fn get_last_scan() -> Option<RootkitScanResult> {
    LAST_SCAN_RESULT.lock().clone()
}

fn scan_loaded_modules() -> Vec<String> {
    let mut out = Vec::new();
    let loaded = crate::security::module_db::get_loaded_modules();
    for m in loaded {
        let trusted = crate::security::module_db::is_trusted_module(&m);
        if !trusted || m.contains("rootkit") || m.contains("stealth") || m.contains("hide") {
            out.push(m);
        }
    }
    out
}

fn scan_filesystem() -> Vec<String> {
    let mut out = Vec::new();
    let files = crate::filesystem::list_hidden_files("/");
    if !files.is_empty() {
        for f in files {
            if f.contains("rootkit") || f.contains(".ko") || f.contains(".so") {
                out.push(f);
            }
        }
    }
    out
}

fn scan_kernel_integrity() -> Vec<String> {
    let mut out = Vec::new();
    if !crate::memory::verify_kernel_data_integrity() {
        out.push("Kernel data section modified".into());
    }
    if !crate::arch::trap::vector_table_is_intact() {
        out.push("Interrupt Descriptor Table modified".into());
    }
    if !crate::memory::verify_kernel_page_tables() {
        out.push("Kernel page tables tampered".into());
    }
    out
}
