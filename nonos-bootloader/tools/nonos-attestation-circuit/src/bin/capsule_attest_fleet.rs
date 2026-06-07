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

#[path = "capsule_attest_fleet/args.rs"]
mod args;
#[path = "capsule_attest_fleet/discover.rs"]
mod discover;
#[path = "capsule_attest_fleet/has_suffix.rs"]
mod has_suffix;
#[path = "capsule_attest_fleet/map_by_name.rs"]
mod map_by_name;
#[path = "capsule_attest_fleet/print_header.rs"]
mod print_header;
#[path = "capsule_attest_fleet/print_pass.rs"]
mod print_pass;
#[path = "capsule_attest_fleet/print_summary.rs"]
mod print_summary;
#[path = "capsule_attest_fleet/require_same_names.rs"]
mod require_same_names;
#[path = "capsule_attest_fleet/run.rs"]
mod run;
#[path = "capsule_attest_fleet/tool_path.rs"]
mod tool_path;
#[path = "capsule_attest_fleet/verify_one.rs"]
mod verify_one;

fn main() -> Result<(), String> {
    run::run()
}
