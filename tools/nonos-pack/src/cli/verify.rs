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

use std::fs;

use super::args::flag;

pub fn run(av: &[String]) -> Result<(), String> {
    let input = flag("verify", av, "--in")?;
    let bytes = fs::read(&input).map_err(|e| format!("{}: {}", input, e))?;
    nonos_pack::sign::verify(&bytes).map_err(|e| format!("verify: {:?}", e))?;
    println!("{}: ed25519 + mldsa65 publisher signatures verify", input);
    Ok(())
}
