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

mod audit;
mod sandbox;
mod threat;

pub use audit::{
    get_high_severity_count, get_latest_event, log_firmware_access, log_security_event,
    AuditResult, SecurityEvent,
};
pub use sandbox::{create_firmware_sandbox, execute_in_sandbox, SandboxConfig, SandboxResult};
pub use threat::{analyze_firmware_behavior, detect_threats, ThreatAnalysis, ThreatLevel};
