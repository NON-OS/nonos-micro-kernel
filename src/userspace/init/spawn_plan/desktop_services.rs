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
    spawn_image_codec();
    spawn_image_viewer();
    spawn_video_player();
    spawn_clipboard();
    spawn_attest();
    spawn_installer();
    spawn_login();
    spawn_toolkit();
}

#[cfg(feature = "nonos-capsule-attest")]
fn spawn_attest() {
    use crate::userspace::capsule_attest as c;
    super::boot::capsule("ATTEST", "attest", c::spawn_attest_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-attest"))]
fn spawn_attest() {}

#[cfg(feature = "nonos-capsule-installer")]
fn spawn_installer() {
    use crate::userspace::capsule_installer as c;
    super::boot::capsule("INSTALLER", "installer", c::spawn_installer_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-installer"))]
fn spawn_installer() {}

#[cfg(feature = "nonos-capsule-image-codec")]
fn spawn_image_codec() {
    use crate::userspace::capsule_image_codec as c;
    super::boot::capsule(
        "IMAGE-CODEC",
        "image_codec",
        c::spawn_image_codec_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-image-codec"))]
fn spawn_image_codec() {}

#[cfg(feature = "nonos-capsule-image-viewer")]
fn spawn_image_viewer() {
    use crate::userspace::capsule_image_viewer as c;
    super::boot::capsule(
        "IMAGE-VIEWER",
        "image_viewer",
        c::spawn_image_viewer_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-image-viewer"))]
fn spawn_image_viewer() {}

#[cfg(feature = "nonos-capsule-video-player")]
fn spawn_video_player() {
    use crate::userspace::capsule_video_player as c;
    super::boot::capsule(
        "VIDEO-PLAYER",
        "video_player",
        c::spawn_video_player_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-video-player"))]
fn spawn_video_player() {}

#[cfg(feature = "nonos-capsule-clipboard")]
fn spawn_clipboard() {
    use crate::userspace::capsule_clipboard as c;
    super::boot::capsule("CLIPBOARD", "clipboard", c::spawn_clipboard_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-clipboard"))]
fn spawn_clipboard() {}

#[cfg(feature = "nonos-capsule-login")]
fn spawn_login() {
    use crate::userspace::capsule_login as c;
    super::boot::capsule("LOGIN", "login", c::spawn_login_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-login"))]
fn spawn_login() {}

#[cfg(feature = "nonos-capsule-toolkit")]
fn spawn_toolkit() {
    use crate::userspace::capsule_toolkit as c;
    super::boot::capsule("TOOLKIT", "toolkit", c::spawn_toolkit_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-toolkit"))]
fn spawn_toolkit() {}
