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
use super::health::HealthStatus;
use crate::firmware::types::FirmwareType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusFlag(pub u32);
#[derive(Debug, Clone)]
pub struct FirmwareStatus {
    pub firmware_type: FirmwareType,
    pub flags: StatusFlag,
    pub health: HealthStatus,
    pub uptime_seconds: u32,
    pub last_error_code: u32,
    pub restart_count: u16,
}
#[derive(Debug, Clone)]
pub struct StatusMonitor {
    statuses: [Option<FirmwareStatus>; 64],
    monitor_interval: u32,
    last_update: u64,
}
static mut GLOBAL_MONITOR: StatusMonitor = StatusMonitor::new();

impl StatusFlag {
    pub const NONE: Self = Self(0);
    pub const LOADED: Self = Self(1);
    pub const ACTIVE: Self = Self(2);
    pub const ERROR: Self = Self(4);
    pub const UPDATING: Self = Self(8);
    pub const DEGRADED: Self = Self(16);
    pub const CRITICAL: Self = Self(32);
    pub fn contains(&self, o: Self) -> bool {
        (self.0 & o.0) == o.0
    }
    pub fn union(&self, o: Self) -> Self {
        Self(self.0 | o.0)
    }
    pub fn remove(&self, o: Self) -> Self {
        Self(self.0 & !o.0)
    }
}
pub fn get_firmware_status(ft: FirmwareType) -> Option<FirmwareStatus> {
    unsafe { GLOBAL_MONITOR.get_status(ft) }
}
pub fn update_status(ft: FirmwareType, fl: StatusFlag, h: HealthStatus) -> bool {
    unsafe { GLOBAL_MONITOR.update_firmware_status(ft, fl, h) }
}
impl StatusMonitor {
    const fn new() -> Self {
        Self { statuses: [const { None }; 64], monitor_interval: 5000, last_update: 0 }
    }
    fn get_status(&self, ft: FirmwareType) -> Option<FirmwareStatus> {
        self.statuses.iter().filter_map(|s| s.as_ref()).find(|s| s.firmware_type == ft).cloned()
    }
    fn update_firmware_status(
        &mut self,
        ft: FirmwareType,
        fl: StatusFlag,
        h: HealthStatus,
    ) -> bool {
        if let Some(e) = self.find_mut(ft) {
            e.flags = fl;
            e.health = h;
            e.uptime_seconds += 1;
            true
        } else {
            self.statuses
                .iter_mut()
                .find(|s| s.is_none())
                .map(|s| {
                    *s = Some(FirmwareStatus::new(ft, fl, h));
                    true
                })
                .unwrap_or(false)
        }
    }
    fn find_mut(&mut self, ft: FirmwareType) -> Option<&mut FirmwareStatus> {
        self.statuses.iter_mut().filter_map(|s| s.as_mut()).find(|s| s.firmware_type == ft)
    }
    pub fn should_poll(&self, t: u64) -> bool {
        t >= self.last_update + self.monitor_interval as u64
    }
}
impl FirmwareStatus {
    fn new(ft: FirmwareType, fl: StatusFlag, h: HealthStatus) -> Self {
        Self {
            firmware_type: ft,
            flags: fl,
            health: h,
            uptime_seconds: 0,
            last_error_code: 0,
            restart_count: 0,
        }
    }
    pub fn is_active(&self) -> bool {
        self.flags.contains(StatusFlag::ACTIVE)
    }
    pub fn has_errors(&self) -> bool {
        self.flags.contains(StatusFlag::ERROR)
    }
    pub fn is_degraded(&self) -> bool {
        self.flags.contains(StatusFlag::DEGRADED) || self.health == HealthStatus::Degraded
    }
    pub fn get_uptime(&self) -> u32 {
        self.uptime_seconds
    }
    pub fn record_error(&mut self, c: u32) {
        self.last_error_code = c;
        self.flags = self.flags.union(StatusFlag::ERROR);
    }
    pub fn record_restart(&mut self) {
        self.restart_count += 1;
        self.uptime_seconds = 0;
    }
}
