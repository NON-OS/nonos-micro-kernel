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

pub fn write_outputs(
    args: &Args,
    proof: &[u8],
    public_inputs: &[u8],
    trailer: &[u8],
    capsule: &[u8],
) -> Result<(), String> {
    fs::write(&args.output, proof).map_err(|e| format!("write proof: {e}"))?;
    if let Some(path) = &args.public_inputs_out {
        fs::write(path, public_inputs).map_err(|e| format!("write public inputs: {e}"))?;
    }
    if let Some(path) = &args.trailer_out {
        fs::write(path, trailer).map_err(|e| format!("write trailer: {e}"))?;
    }
    if let Some(path) = &args.capsule_with_trailer_out {
        let mut out = capsule.to_vec();
        out.extend_from_slice(trailer);
        fs::write(path, out).map_err(|e| format!("write capsule trailer image: {e}"))?;
    }
    Ok(())
}
