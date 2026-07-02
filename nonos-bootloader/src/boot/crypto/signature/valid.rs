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

use crate::display::{log_ok, update_stage, StageStatus, STAGE_ED25519_VERIFY};
use crate::security::{audit, set_signature_attestation, AuditEvent};

pub fn handle_valid_signature(gop: bool) {
    update_stage(STAGE_ED25519_VERIFY, StageStatus::Success);
    set_signature_attestation(true);
    audit(AuditEvent::SignatureVerified, 0, b"sig valid");
    if gop {
        log_ok(b"kernel signature VALID");
    }
}
