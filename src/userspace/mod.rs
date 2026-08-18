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

// Microkernel runtime: init bootstrap and the kernel-side mirrors
// for every userland capsule the boot path spawns. Real capsule
// binaries live under `userland/<name>/`; the mirror here only
// carries the signed embed bytes (ELF + manifest + cert), the
// spawn entry, and liveness state. No protocol logic lives in
// the kernel — that runs inside the spawned capsule.
//
// Kernel-resident `*_engine` wrappers live under `src/services/`
// and are not real userspace. The CI grep gate in
// `nonos-ci/run-static-checks.sh` rejects any new
// `src/userspace/*_service` directory.

pub mod capsule_about;
pub mod capsule_attest;
pub mod capsule_audio_player;
pub mod capsule_boot_splash;
pub mod capsule_browser;
pub mod capsule_calculator;
pub mod capsule_clipboard;
pub mod capsule_clock;
pub mod capsule_compositor;
pub mod capsule_desktop_shell;
pub mod capsule_driver_i2c_hid;
pub mod capsule_driver_usb_hid;
pub mod capsule_driver_usb_msc;
#[cfg(feature = "nonos-capsule-flacprobe")]
pub mod capsule_flacprobe;
pub mod capsule_file_manager;
pub mod capsule_hello;
pub mod capsule_image_codec;
pub mod capsule_image_viewer;
#[cfg(feature = "nonos-capsule-input-probe")]
pub mod capsule_input_probe;
pub mod capsule_input_proof;
pub mod capsule_input_router;
pub mod capsule_installer;
pub mod capsule_login;
pub mod capsule_net_core;
pub mod capsule_net_dhcp;
pub mod capsule_net_dns;
pub mod capsule_net_ip;
pub mod capsule_net_l2;
#[cfg(feature = "nonos-capsule-net-ntp")]
pub mod capsule_net_ntp;
pub mod capsule_net_nym;
pub mod capsule_socks5;
pub mod capsule_net_sockets;
pub mod capsule_net_tcp;
pub mod capsule_net_udp;
pub mod capsule_policy;
pub mod capsule_process_manager;
pub mod capsule_proof_io;
pub mod capsule_ripgrep;
pub mod capsule_sd;
pub mod capsule_settings;
#[cfg(feature = "nonos-capsule-setup-wizard")]
pub mod capsule_setup_wizard;
pub mod capsule_snake;
pub mod capsule_std_proof;
pub mod capsule_terminal;
pub mod capsule_text_editor;
pub mod capsule_tokio_smoke;
pub mod capsule_toolkit;
pub mod capsule_video_player;
pub mod capsule_wallet_nonos;
pub mod capsule_wallpaper;
pub mod capsule_wallpaper_catalog;
pub mod capsule_wm;
pub mod init;
pub mod tool_capsules;

pub use init::run_init;
