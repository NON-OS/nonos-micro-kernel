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

use nonos_attestation_circuit::{CeremonyTranscript, ContributionRecord};

pub fn find_record(
    tx: &CeremonyTranscript,
    round: u32,
) -> Result<ContributionRecord, Box<dyn std::error::Error>> {
    if !tx.metadata.finalized || !tx.verification_passed {
        return Err("ceremony transcript is not finalized and verified".into());
    }
    tx.contributions
        .iter()
        .find(|r| r.round == round)
        .cloned()
        .ok_or_else(|| format!("round {round} not found in transcript").into())
}
