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

use crate::hardware::tpm::state::TpmState;

/// Hand the ReadPublic command to the TPM.
///
/// Goes through the shared transport rather than driving registers here. This
/// wrote the FIFO sequence out again by hand, so the EK read spoke FIFO
/// whatever the part actually presented and could never work on a CRB TPM even
/// once the rest of the driver did. One transport, chosen from the detected
/// interface, is the only arrangement where the two cannot drift apart.
pub fn send_read_public(state: &TpmState, cmd: &[u8]) -> Result<(), &'static str> {
    state.send_command(cmd).map_err(|_| "TPM command submit failed")
}
