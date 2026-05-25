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

pub(super) fn spawn() {
    spawn_input_proof();
    spawn_about();
    spawn_calculator();
    spawn_terminal();
    spawn_file_manager();
    super::apps_tools::spawn();
}

#[cfg(feature = "nonos-capsule-input-proof")]
fn spawn_input_proof() {
    use crate::userspace::capsule_input_proof as c;
    super::boot::capsule(
        "APP-INPUT-PROOF",
        "app_input_proof",
        c::spawn_input_proof_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-input-proof"))]
fn spawn_input_proof() {}

#[cfg(feature = "nonos-capsule-about")]
fn spawn_about() {
    use crate::userspace::capsule_about as c;
    super::boot::capsule("APP-ABOUT", "app_about", c::spawn_about_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-about"))]
fn spawn_about() {}

#[cfg(feature = "nonos-capsule-calculator")]
fn spawn_calculator() {
    use crate::userspace::capsule_calculator as c;
    super::boot::capsule(
        "APP-CALCULATOR",
        "app_calculator",
        c::spawn_calculator_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-calculator"))]
fn spawn_calculator() {}

#[cfg(feature = "nonos-capsule-terminal")]
fn spawn_terminal() {
    use crate::userspace::capsule_terminal as c;
    super::boot::capsule(
        "APP-TERMINAL",
        "app_terminal",
        c::spawn_terminal_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-terminal"))]
fn spawn_terminal() {}

#[cfg(feature = "nonos-capsule-file-manager")]
fn spawn_file_manager() {
    use crate::userspace::capsule_file_manager as c;
    super::boot::capsule(
        "APP-FILE-MANAGER",
        "app_file_manager",
        c::spawn_file_manager_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-file-manager"))]
fn spawn_file_manager() {}
