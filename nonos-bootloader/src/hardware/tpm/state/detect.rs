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

use super::core::TpmState;
use crate::hardware::tpm::constants::{
    TPM_CRB_VID_DID, TPM_DID_VID, TPM_INTERFACE_ID, TPM_INTF_TYPE_CRB, TPM_INTF_TYPE_MASK,
};
use crate::hardware::tpm::types::TpmError;

/// A word carrying no device: an unimplemented register reads back zero, an
/// absent decode all ones.
fn absent(word: u32) -> bool {
    word == 0 || word == 0xFFFF_FFFF
}

impl TpmState {
    /// Look for a TPM at the fixed MMIO window and record which register file
    /// it presents.
    ///
    /// The interface has to be identified before identity can be read, because
    /// the two register files keep identity in different places. This read
    /// `TPM_DID_VID` at 0x0F00 unconditionally, which is the FIFO location: a
    /// CRB part answers zero there and was reported as no TPM at all. That is
    /// not a corner case but most current hardware, since Intel PTT and AMD
    /// fTPM are both CRB, so the bootloader skipped the TPM on any machine
    /// with a firmware TPM and fell back to a software machine id.
    pub fn detect(&mut self) -> Result<bool, TpmError> {
        let interface_id = self.read_reg32(TPM_INTERFACE_ID);
        let is_crb =
            !absent(interface_id) && (interface_id & TPM_INTF_TYPE_MASK) == TPM_INTF_TYPE_CRB;

        // Identity comes out of the file the interface claims. Where the
        // interface register itself is unreadable, probing the FIFO file
        // directly is what an older part without one looks like.
        let did_vid =
            if is_crb { self.read_reg32(TPM_CRB_VID_DID) } else { self.read_reg32(TPM_DID_VID) };

        if absent(did_vid) {
            return Ok(false);
        }

        // A CRB file only ever carries a 2.0 part. For FIFO the distinction is
        // real, and an unreadable interface register is how 1.2 presents.
        self.version = if is_crb || !absent(interface_id) { 20 } else { 12 };
        self.is_crb = is_crb;

        self.initialized = true;
        Ok(true)
    }
}
