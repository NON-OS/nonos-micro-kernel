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

use std::fs;

use super::args::Args;
use super::input::ProofInput;

pub fn read_file_input(args: &Args) -> Result<ProofInput, String> {
    let proof = args.proof.as_ref().ok_or("missing --proof")?;
    let public_inputs = args.public_inputs.as_ref().ok_or("missing --public-inputs")?;
    Ok(ProofInput {
        proof: fs::read(proof).map_err(|e| format!("read proof: {e}"))?,
        public_inputs: fs::read(public_inputs).map_err(|e| format!("read public inputs: {e}"))?,
        trailer_commitment: None,
        body_hash: None,
    })
}
