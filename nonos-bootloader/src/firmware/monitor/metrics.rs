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

use crate::firmware::types::FirmwareType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Timer,
}
#[derive(Clone, Copy)]
pub enum MetricValue {
    Counter(u64),
    Gauge(i64),
    Timer(u64),
}
#[derive(Debug, Clone, Copy)]
pub struct FirmwareMetrics {
    firmware_type: FirmwareType,
    counters: [u64; 8],
    gauges: [i64; 8],
    last_update: u64,
}
impl FirmwareMetrics {
    const fn empty() -> Self {
        Self {
            firmware_type: FirmwareType::Unknown,
            counters: [0; 8],
            gauges: [0; 8],
            last_update: 0,
        }
    }
    fn new(ft: FirmwareType) -> Self {
        Self { firmware_type: ft, counters: [0; 8], gauges: [0; 8], last_update: ts() }
    }
    pub fn get_counter(&self, i: usize) -> u64 {
        if i < 8 {
            self.counters[i]
        } else {
            0
        }
    }
    pub fn get_gauge(&self, i: usize) -> i64 {
        if i < 8 {
            self.gauges[i]
        } else {
            0
        }
    }
}
static mut GLOBAL_METRICS: [FirmwareMetrics; 32] = [const { FirmwareMetrics::empty() }; 32];
static mut METRICS_COUNT: usize = 0;

pub fn collect_metrics(ft: FirmwareType) -> Option<&'static FirmwareMetrics> {
    unsafe { GLOBAL_METRICS.iter().find(|m| m.firmware_type == ft && m.last_update > 0) }
}

pub fn update_metric(ft: FirmwareType, name: &str, mt: MetricType, val: u64) -> bool {
    let idx = name.bytes().fold(0usize, |a, b| a.wrapping_add(b as usize)) % 8;
    unsafe {
        if let Some(m) = GLOBAL_METRICS.iter_mut().find(|m| m.firmware_type == ft) {
            match mt {
                MetricType::Counter => m.counters[idx] = val,
                MetricType::Gauge => m.gauges[idx] = val as i64,
                _ => m.counters[idx] = val,
            }
            m.last_update = ts();
            return true;
        }
        if METRICS_COUNT < GLOBAL_METRICS.len() {
            let mut nm = FirmwareMetrics::new(ft);
            match mt {
                MetricType::Counter => nm.counters[idx] = val,
                MetricType::Gauge => nm.gauges[idx] = val as i64,
                _ => nm.counters[idx] = val,
            }
            GLOBAL_METRICS[METRICS_COUNT] = nm;
            METRICS_COUNT += 1;
            return true;
        }
    }
    false
}

fn ts() -> u64 {
    static mut C: u64 = 0;
    unsafe {
        C += 1;
        C
    }
}
