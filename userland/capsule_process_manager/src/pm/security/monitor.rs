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

use alloc::vec::Vec;

use super::super::state::Row;
use super::posture::Posture;
use super::sensitive::ADMIN;
use super::types::{Alert, Level};
use super::watchlist::{service_name, watched_index, SERVICE_COUNT};

// A process is treated as pinned once it holds at least this share of the cpu,
// and is only flagged after it has stayed there for this many refreshes, so a
// brief burst never raises an alert.
const SATURATION_PCT: u8 = 95;
const SUSTAIN_SAMPLES: u8 = 3;

// Stateful security view over the live process table. It remembers which
// watched services it has seen running and how long each process has been
// saturating the cpu, so its findings reflect change over time, not a single
// frame. Every finding is derived from real kernel data.
#[derive(Default)]
pub struct Monitor {
    // Watched services observed running at least once (bit per service index).
    seen_mask: u32,
    // (pid, consecutive refreshes at or above saturation).
    streak: Vec<(u32, u8)>,
    pub posture: Posture,
}

impl Monitor {
    pub fn new() -> Self {
        Monitor::default()
    }

    // Recompute posture and findings from the current rows, updating the
    // cross-refresh memory. Findings are returned worst-first.
    pub fn evaluate(&mut self, rows: &[Row]) -> Vec<Alert> {
        self.posture = Posture::compute(rows);
        let mut alerts = Vec::new();

        self.check_missing_services(rows, &mut alerts);
        self.check_saturation(rows, &mut alerts);
        check_admin_holders(rows, &mut alerts);

        alerts.sort_by(|a, b| b.level.cmp(&a.level).then(a.pid.cmp(&b.pid)));
        alerts
    }

    // A watched service that was seen running and is now absent is a real,
    // security-relevant event: the session lost a service it depends on.
    fn check_missing_services(&mut self, rows: &[Row], alerts: &mut Vec<Alert>) {
        let mut present = 0u32;
        for r in rows {
            if let Some(i) = watched_index(r.name()) {
                present |= 1 << i;
            }
        }
        let missing = self.seen_mask & !present;
        self.seen_mask |= present;
        for i in 0..SERVICE_COUNT {
            if missing & (1 << i) != 0 {
                alerts.push(Alert::about(
                    Level::Critical,
                    0,
                    service_name(i),
                    b"watched service is no longer running",
                ));
            }
        }
    }

    // A process holding the cpu near fully for several samples in a row is a
    // runaway or an unexpected heavy workload worth surfacing.
    fn check_saturation(&mut self, rows: &[Row], alerts: &mut Vec<Alert>) {
        let mut next = Vec::new();
        for r in rows {
            let prev = self.streak.iter().find(|(p, _)| *p == r.pid).map(|(_, c)| *c).unwrap_or(0);
            let count = if r.cpu_pct >= SATURATION_PCT { prev.saturating_add(1) } else { 0 };
            if count > 0 {
                next.push((r.pid, count));
            }
            if count >= SUSTAIN_SAMPLES {
                alerts.push(Alert::about(
                    Level::Warn,
                    r.pid,
                    r.name(),
                    b"pinned at full cpu across several samples",
                ));
            }
        }
        self.streak = next;
    }
}

// Admin authority grants the whole system. Nothing but the init process should
// normally hold it, so every other holder is surfaced for review.
fn check_admin_holders(rows: &[Row], alerts: &mut Vec<Alert>) {
    for r in rows {
        if r.caps & ADMIN != 0 && r.name() != b"init" {
            alerts.push(Alert::about(Level::Info, r.pid, r.name(), b"holds admin authority"));
        }
    }
}
