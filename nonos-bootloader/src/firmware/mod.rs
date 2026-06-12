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

pub mod cache;
pub mod detection;
pub mod error;
mod loader;
pub mod monitor;
pub mod quirks;
pub mod registry;
pub mod security;
mod types;
pub mod validation;

pub use cache::{cache_firmware, compress_firmware, CacheResult, CompressionType, MemoryCache};
pub use detection::{
    check_firmware_compatibility, detect_hardware_devices, parse_firmware_version,
    CompatibilityResult, HardwareDevice,
};
pub use error::{
    attempt_error_recovery, report_error, ErrorSeverity, FirmwareError, RecoveryStrategy,
};
pub use loader::{firmware_count, get_firmware, get_firmware_handoff, has_embedded_firmware};
pub use monitor::{
    check_firmware_health, collect_metrics, get_firmware_status, FirmwareMetrics, HealthStatus,
};
pub use quirks::{apply_mmap_quirks, detect_firmware_quirks, FirmwareQuirk, QuirkFlags};
pub use registry::{
    register_firmware, search_firmware, FirmwareDatabase, FirmwareMetadata, SearchQuery,
};
pub use security::{
    create_firmware_sandbox, detect_threats, log_security_event, SandboxConfig, ThreatLevel,
};
pub use types::{FirmwareEntry, FirmwareHandoff, FirmwareType, MAX_FIRMWARE_ENTRIES};
pub use validation::{
    calculate_sha256, validate_firmware_integrity, verify_signature, IntegrityResult,
    SignatureResult,
};
