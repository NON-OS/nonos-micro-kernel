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

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub memory_limit: u64,
    pub timeout_ms: u32,
    pub allow_network: bool,
    pub allow_storage: bool,
    pub isolation_level: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxResult {
    Success,
    MemoryExceeded,
    TimeoutExceeded,
    AccessViolation,
    ExecutionError,
    InvalidConfig,
}
impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            memory_limit: 64 * 1024 * 1024,
            timeout_ms: 5000,
            allow_network: false,
            allow_storage: false,
            isolation_level: 2,
        }
    }
}

pub fn create_firmware_sandbox(cfg: SandboxConfig) -> Result<u32, SandboxResult> {
    if cfg.memory_limit == 0
        || cfg.timeout_ms == 0
        || cfg.memory_limit > 1024 * 1024 * 1024
        || cfg.isolation_level > 3
    {
        return Err(SandboxResult::InvalidConfig);
    }
    static mut NEXT_ID: u32 = 1;
    let sid = unsafe {
        NEXT_ID += 1;
        NEXT_ID - 1
    };
    if cfg.memory_limit > 1024 * 1024 * 1024 {
        return Err(SandboxResult::MemoryExceeded);
    }
    if cfg.timeout_ms > 30000 {
        return Err(SandboxResult::TimeoutExceeded);
    }
    Ok(sid)
}

pub fn execute_in_sandbox(sid: u32, data: &[u8]) -> SandboxResult {
    if sid == 0 || sid >= 65536 {
        return SandboxResult::InvalidConfig;
    }
    if data.is_empty() || data.len() > 16 * 1024 * 1024 {
        return SandboxResult::MemoryExceeded;
    }
    SandboxResult::Success
}
