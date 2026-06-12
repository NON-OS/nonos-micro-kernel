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

use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::types::LoadedSegment;
use crate::loader::validate::ValidationContext;
use crate::log::logger::{log_error, log_info, log_warn};

use super::policy::{SecurityCheckResult, SecurityPolicy};

pub use super::hash::{compute_kernel_hash, verify_kernel_hash};

pub fn check_wx_policy(
    segments: &[Option<LoadedSegment>],
    policy: &SecurityPolicy,
) -> LoaderResult<usize> {
    let mut violations = 0;

    for segment in segments.iter().flatten() {
        if segment.has_wx() {
            violations += 1;
            log_warn("security", "W^X violation: segment is both writable and executable");
        }
    }

    if violations > 0 && policy.enforce_wx {
        log_error("security", "SECURITY: W^X policy violated - aborting load");
        return Err(LoaderError::WxViolation);
    }

    Ok(violations)
}

pub fn check_address_bounds(
    segments: &[Option<LoadedSegment>],
    policy: &SecurityPolicy,
) -> LoaderResult<()> {
    for segment in segments.iter().flatten() {
        let start = segment.target_addr;
        let end = start.checked_add(segment.mem_size).ok_or(LoaderError::IntegerOverflow)?;

        if start == 0 {
            continue;
        }

        if start < policy.min_load_address {
            log_error("security", "SECURITY: Segment address below minimum allowed");
            return Err(LoaderError::AddressOutOfRange);
        }

        if end > policy.max_load_address {
            log_error("security", "SECURITY: Segment extends beyond maximum allowed address");
            return Err(LoaderError::AddressOutOfRange);
        }
    }

    Ok(())
}

pub fn check_size_policy(total_size: usize, policy: &SecurityPolicy) -> LoaderResult<()> {
    if total_size > policy.max_kernel_size {
        log_error("security", "SECURITY: Kernel exceeds maximum allowed size");
        return Err(LoaderError::KernelTooLarge);
    }

    if total_size == 0 {
        log_error("security", "SECURITY: Kernel has zero size");
        return Err(LoaderError::MalformedElf("zero size kernel"));
    }

    Ok(())
}

pub fn check_pie_policy(ctx: &ValidationContext, policy: &SecurityPolicy) -> LoaderResult<()> {
    if policy.require_pie && !ctx.is_pie {
        log_error("security", "SECURITY: PIE required but kernel is not position-independent");
        return Err(LoaderError::UnsupportedElf("PIE required"));
    }

    Ok(())
}

pub fn check_critical_memory(segments: &[Option<LoadedSegment>]) -> LoaderResult<()> {
    const CRITICAL_REGIONS: &[(u64, u64, &str)] = &[
        (0x0, 0x1000, "real mode IVT"),
        (0x80000, 0xA0000, "EBDA area"),
        (0xA0000, 0x100000, "video memory and ROM"),
    ];

    for segment in segments.iter().flatten() {
        if segment.target_addr == 0 {
            continue;
        }

        let seg_start = segment.target_addr;
        let seg_end = seg_start + segment.mem_size;

        for (crit_start, crit_end, _name) in CRITICAL_REGIONS {
            if seg_start < *crit_end && *crit_start < seg_end {
                log_error("security", "SECURITY: Segment overlaps critical memory region");
                return Err(LoaderError::AddressOutOfRange);
            }
        }
    }

    Ok(())
}

pub fn validate_security(
    ctx: &ValidationContext,
    segments: &[Option<LoadedSegment>],
    policy: &SecurityPolicy,
) -> LoaderResult<SecurityCheckResult> {
    let mut result = SecurityCheckResult::default();

    check_size_policy(ctx.total_size, policy)?;
    check_address_bounds(segments, policy)?;
    check_critical_memory(segments)?;
    result.wx_violations = check_wx_policy(segments, policy)?;
    check_pie_policy(ctx, policy)?;

    if result.wx_violations > 0 && !policy.enforce_wx {
        result.warnings += result.wx_violations;
    }

    result.passed = !result.has_violations();

    if result.passed {
        log_info("security", "Security validation passed");
    }

    Ok(result)
}
