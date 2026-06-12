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

/*
 * Boot Watchdog.
 *
 * Software watchdog to detect boot stage hangs:
 * - Tracks current boot stage
 * - Records stage timestamps
 * - Detects stuck stages
 *
 * Used by Issue #8 workaround for ExitBootServices hangs.
 */

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

const MAX_BOOT_STAGES: usize = 16;

static CURRENT_STAGE: AtomicU32 = AtomicU32::new(0);
static STAGE_START: AtomicU64 = AtomicU64::new(0);
static mut STAGE_TIMESTAMPS: [u64; MAX_BOOT_STAGES] = [0; MAX_BOOT_STAGES];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BootStage {
    Init = 0,
    UefiInit = 1,
    SecurityChecks = 2,
    HardwareDiscovery = 3,
    KernelLoad = 4,
    CryptoVerify = 5,
    ZkAttestation = 6,
    ElfParse = 7,
    HandoffPrepare = 8,
    MemoryMapGet = 9,
    ExitBootServices = 10,
    KernelEntry = 11,
    Complete = 15,
}

pub fn enter_stage(stage: BootStage) {
    let ts = read_tsc();
    let stage_num = stage as u32;

    CURRENT_STAGE.store(stage_num, Ordering::SeqCst);
    STAGE_START.store(ts, Ordering::SeqCst);

    if (stage_num as usize) < MAX_BOOT_STAGES {
        unsafe {
            STAGE_TIMESTAMPS[stage_num as usize] = ts;
        }
    }
}

pub fn get_current_stage() -> u32 {
    CURRENT_STAGE.load(Ordering::SeqCst)
}

pub fn get_stage_duration(stage: BootStage) -> Option<u64> {
    let stage_num = stage as usize;
    if stage_num >= MAX_BOOT_STAGES - 1 {
        return None;
    }

    unsafe {
        let start = STAGE_TIMESTAMPS[stage_num];
        let end = STAGE_TIMESTAMPS[stage_num + 1];

        if start == 0 || end == 0 {
            None
        } else {
            Some(end.saturating_sub(start))
        }
    }
}

pub fn stage_elapsed() -> u64 {
    let start = STAGE_START.load(Ordering::SeqCst);
    let now = read_tsc();
    now.saturating_sub(start)
}

pub fn check_stage_timeout(max_cycles: u64) -> bool {
    stage_elapsed() > max_cycles
}

fn read_tsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_rdtsc()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

pub struct StageGuard {
    _stage: BootStage,
}

impl StageGuard {
    pub fn new(stage: BootStage) -> Self {
        enter_stage(stage);
        Self { _stage: stage }
    }
}

impl Drop for StageGuard {
    fn drop(&mut self) {}
}
