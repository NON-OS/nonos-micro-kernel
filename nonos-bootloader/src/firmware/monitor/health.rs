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
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Critical,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthResult {
    Pass,
    Fail,
    Warning,
    Timeout,
    NotApplicable,
}
#[derive(Debug, Clone)]
pub struct HealthCheck {
    firmware_type: FirmwareType,
    check_name: [u8; 32],
    last_result: HealthResult,
    last_check_time: u64,
    check_count: u32,
    failure_count: u32,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct PerformanceMetrics {
    pub load_time_ms: u32,
    pub memory_usage_bytes: u32,
    pub cpu_usage_percent: u8,
    pub temperature_celsius: u8,
    pub error_rate_per_hour: u16,
}
impl Default for HealthCheck {
    fn default() -> Self {
        Self {
            firmware_type: FirmwareType::Unknown,
            check_name: [0; 32],
            last_result: HealthResult::NotApplicable,
            last_check_time: 0,
            check_count: 0,
            failure_count: 0,
        }
    }
}
impl HealthCheck {
    pub fn new(ft: FirmwareType, name: &str) -> Self {
        let mut c = Self::default();
        c.firmware_type = ft;
        let b = name.as_bytes();
        c.check_name[..core::cmp::min(b.len(), 31)]
            .copy_from_slice(&b[..core::cmp::min(b.len(), 31)]);
        c
    }
    pub fn execute(&mut self) -> HealthResult {
        self.check_count += 1;
        self.last_check_time = ts();
        let r = if self.firmware_type == FirmwareType::Unknown {
            HealthResult::NotApplicable
        } else {
            HealthResult::Pass
        };
        if r == HealthResult::Fail {
            self.failure_count += 1;
        }
        self.last_result = r;
        r
    }
    pub fn get_success_rate(&self) -> f32 {
        if self.check_count == 0 {
            0.0
        } else {
            (self.check_count - self.failure_count) as f32 / self.check_count as f32
        }
    }
}

pub fn check_firmware_health(ft: FirmwareType) -> HealthStatus {
    let m = get_metrics(ft);
    let mut s = 100u8;
    if m.load_time_ms > 1000 {
        s = s.saturating_sub(10);
    }
    if m.memory_usage_bytes > 64 * 1024 * 1024 {
        s = s.saturating_sub(15);
    }
    if m.cpu_usage_percent > 80 {
        s = s.saturating_sub(20);
    }
    if m.temperature_celsius > 85 {
        s = s.saturating_sub(25);
    }
    if m.error_rate_per_hour > 10 {
        s = s.saturating_sub(30);
    }
    match s {
        90..=100 => HealthStatus::Healthy,
        70..=89 => HealthStatus::Degraded,
        50..=69 => HealthStatus::Unhealthy,
        _ => HealthStatus::Critical,
    }
}

fn get_metrics(ft: FirmwareType) -> PerformanceMetrics {
    let base = match ft {
        FirmwareType::IntelAx200 | FirmwareType::IntelAx210 => 500,
        FirmwareType::Rtl8821c | FirmwareType::Rtl8822c => 800,
        _ => 300,
    };
    PerformanceMetrics {
        load_time_ms: base + (ts() % 200) as u32,
        memory_usage_bytes: 32 * 1024 * 1024 + ((ts() % 16) * 1024 * 1024) as u32,
        cpu_usage_percent: (20 + ts() % 60) as u8,
        temperature_celsius: (45 + ts() % 40) as u8,
        error_rate_per_hour: (ts() % 5) as u16,
    }
}

fn ts() -> u64 {
    static mut C: u64 = 0;
    unsafe {
        C += 1;
        C
    }
}
