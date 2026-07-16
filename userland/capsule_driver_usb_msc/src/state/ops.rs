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

use crate::bot::{validate, CommandStatus, TransferOutcome, ValidateError};
use crate::descriptors::ProbeResult;
use crate::protocol::{E_INVAL, E_PHASE};

use super::types::State;

impl State {
    pub fn install_bindings(&mut self, probe: &ProbeResult) {
        self.bindings = probe.bindings;
        self.binding_count = probe.count;
        self.probes = self.probes.saturating_add(1);
    }

    fn next_tag(&mut self) -> u32 {
        let tag = self.next_tag;
        self.next_tag = self.next_tag.wrapping_add(1).max(1);
        self.last_tag = tag;
        tag
    }

    /// Open a command block: assign it a fresh tag, record how many data bytes
    /// it asks for, and mark a transfer as outstanding. The tag and length are
    /// what the returning status wrapper is later checked against.
    pub fn begin_command(&mut self, data_len: u32) -> u32 {
        self.last_data_len = data_len;
        self.pending = true;
        self.next_tag()
    }

    /// Close a command block against its status wrapper. The CSW is validated
    /// (signature and range by the parser, tag echo and residue here) before it
    /// is trusted. Returns the SCSI-level status (0 passed, 1 CHECK CONDITION)
    /// or a protocol error if the wrapper does not belong to this command.
    pub fn finish_command(&mut self, csw: CommandStatus) -> Result<u8, i32> {
        self.pending = false;
        self.residue_bytes = self.residue_bytes.saturating_add(csw.residue as u64);
        match validate(csw, self.last_tag, self.last_data_len) {
            Ok(TransferOutcome::Passed { .. }) => {
                self.csw_ok = self.csw_ok.saturating_add(1);
                Ok(0)
            }
            Ok(TransferOutcome::Failed { .. }) => {
                self.csw_failed = self.csw_failed.saturating_add(1);
                Ok(1)
            }
            Err(ValidateError::TagMismatch) => {
                self.phase_errors = self.phase_errors.saturating_add(1);
                Err(E_INVAL)
            }
            Err(ValidateError::PhaseError) => {
                self.phase_errors = self.phase_errors.saturating_add(1);
                Err(E_PHASE)
            }
        }
    }
}
