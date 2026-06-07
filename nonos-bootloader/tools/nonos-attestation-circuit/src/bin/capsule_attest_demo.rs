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

#[path = "capsule_attest_demo/args.rs"]
mod args;
#[path = "capsule_attest_demo/default_capsule.rs"]
mod default_capsule;
#[path = "capsule_attest_demo/ensure_capsule.rs"]
mod ensure_capsule;
#[path = "capsule_attest_demo/generate_keys.rs"]
mod generate_keys;
#[path = "capsule_attest_demo/generate_proof.rs"]
mod generate_proof;
#[path = "capsule_attest_demo/print_header.rs"]
mod print_header;
#[path = "capsule_attest_demo/print_proof_lines.rs"]
mod print_proof_lines;
#[path = "capsule_attest_demo/require_tool.rs"]
mod require_tool;
#[path = "capsule_attest_demo/run.rs"]
mod run;
#[path = "capsule_attest_demo/run_output.rs"]
mod run_output;
#[path = "capsule_attest_demo/run_status.rs"]
mod run_status;
#[path = "capsule_attest_demo/tamper.rs"]
mod tamper;
#[path = "capsule_attest_demo/temp_root.rs"]
mod temp_root;
#[path = "capsule_attest_demo/tool_path.rs"]
mod tool_path;
#[path = "capsule_attest_demo/verify_capsule.rs"]
mod verify_capsule;
#[path = "capsule_attest_demo/verify_tampered.rs"]
mod verify_tampered;
#[path = "capsule_attest_demo/verify_valid.rs"]
mod verify_valid;

fn main() -> Result<(), String> {
    run::run()
}
