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
    spawn_hello();
    spawn_calculator();
    spawn_clock();
    spawn_browser();
    spawn_snake();
    spawn_wallet_nonos();
    spawn_terminal();
    spawn_file_manager();
    spawn_audio_player();
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

#[cfg(feature = "nonos-capsule-hello")]
fn spawn_hello() {
    use crate::userspace::capsule_hello as c;
    super::boot::capsule("APP-HELLO", "app_hello", c::spawn_hello_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-hello"))]
fn spawn_hello() {}

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

#[cfg(feature = "nonos-capsule-clock")]
fn spawn_clock() {
    use crate::userspace::capsule_clock as c;
    super::boot::capsule("APP-CLOCK", "app_clock", c::spawn_clock_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-clock"))]
fn spawn_clock() {}

#[cfg(feature = "nonos-capsule-browser")]
fn spawn_browser() {
    use crate::userspace::capsule_browser as b;
    super::boot::capsule("APP-BROWSER", "app_browser", b::spawn_browser_capsule, b::shared_state);
}
#[cfg(not(feature = "nonos-capsule-browser"))]
fn spawn_browser() {}

#[cfg(feature = "nonos-capsule-wallet-nonos")]
fn spawn_wallet_nonos() {
    use crate::userspace::capsule_wallet_nonos as c;
    super::boot::capsule(
        "APP-NONOS-WALLET",
        "app_nonos_wallet",
        c::spawn_wallet_nonos_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-wallet-nonos"))]
fn spawn_wallet_nonos() {}

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

#[cfg(feature = "nonos-capsule-audio-player")]
fn spawn_audio_player() {
    use crate::userspace::capsule_audio_player as c;
    super::boot::capsule(
        "APP-AUDIO-PLAYER",
        "app_audio_player",
        c::spawn_audio_player_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-audio-player"))]
fn spawn_audio_player() {}

#[cfg(feature = "nonos-capsule-snake")]
fn spawn_snake() {
    use crate::userspace::capsule_snake as c;
    super::boot::capsule("APP-SNAKE", "app_snake", c::spawn_snake_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-snake"))]
fn spawn_snake() {}
