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

use crate::bot::CommandStatus;

/// The result of a status wrapper that passed validation. `transferred` is how
/// many of the requested data bytes actually moved, derived from the residue.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransferOutcome {
    /// The command succeeded (CSW status 0x00).
    Passed { transferred: u32 },
    /// The command failed with a CHECK CONDITION (CSW status 0x01); the caller
    /// must issue REQUEST SENSE to learn why.
    Failed { transferred: u32 },
}

/// Why a status wrapper was rejected. Both mean the transport is out of sync
/// with the device and the bulk endpoints must be reset before continuing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ValidateError {
    /// dCSWTag did not echo the tag of the command block it answers. The status
    /// belongs to a different command and must never be trusted.
    TagMismatch,
    /// A phase error (CSW status 0x02) or a residue larger than the amount that
    /// was requested: the device could not honour the command block wrapper.
    PhaseError,
}

/// Validate a parsed CSW against the command it is meant to answer, per the USB
/// Mass Storage Bulk-Only Transport rules (§6.3). The signature and a
/// status-in-range check are already enforced by the parser; this pins the two
/// remaining invariants a codec must not skip: the tag echoes the command, and
/// the residue never exceeds the requested length.
pub fn validate(
    csw: CommandStatus,
    expected_tag: u32,
    expected_data_len: u32,
) -> Result<TransferOutcome, ValidateError> {
    if csw.tag != expected_tag {
        return Err(ValidateError::TagMismatch);
    }
    if csw.status == 2 || csw.residue > expected_data_len {
        return Err(ValidateError::PhaseError);
    }
    let transferred = expected_data_len - csw.residue;
    match csw.status {
        0 => Ok(TransferOutcome::Passed { transferred }),
        _ => Ok(TransferOutcome::Failed { transferred }),
    }
}
