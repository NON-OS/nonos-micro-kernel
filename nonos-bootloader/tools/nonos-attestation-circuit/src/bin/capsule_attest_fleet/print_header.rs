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

pub fn print_header() {
    println!("NØNOS capsule attestation fleet verification");
    println!("curve: BLS12-381");
    println!("proof_system: Groth16");
    println!("proof_size: 192 bytes");
    println!("public_inputs: 7 BLS12-381 field elements");
    println!("layout: capsule_hash_hi, capsule_hash_lo, program_hash_hi, program_hash_lo, caps, commitment_hi, commitment_lo");
    println!("binding: proof public inputs + blake3(real capsule bytes) + exact cap mask");
    println!("capsules:");
}
