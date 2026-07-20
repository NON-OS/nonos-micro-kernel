# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
CAPSULE_SLUG               := tokio-smoke
CAPSULE_HANDLE             := tokio_smoke
CAPSULE_DIR                := userland/capsule_tokio_smoke
CAPSULE_BIN_NAME           := tokio-smoke
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_NAMESPACE          := systems.nonos.tokio_smoke
CAPSULE_SERVICE_ENDPOINT   := service:4504:tokio_smoke
CAPSULE_REPLY_ENDPOINT     := reply:4505:endpoint.tokio_smoke.reply
CAPSULE_REQUIRED_CAPS      := 0x11d
CAPSULE_CAPS_CEILING       := 0x11d
CAPSULE_PREBUILT_BIN       := target/upstream-tokio-smoke/tokio-smoke
CAPSULE_METADATA           := nonos tokio-smoke v0.0.0 publisher

include nonos-mk/capsule.mk
