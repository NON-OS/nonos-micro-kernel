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
CAPSULE_SLUG               := sd
CAPSULE_HANDLE             := sd
CAPSULE_DIR                := userland/capsule_sd
CAPSULE_BIN_NAME           := sd
CAPSULE_DOMAIN             := crates.io
CAPSULE_NAMESPACE          := systems.nonos.tool.sd
CAPSULE_SERVICE_ENDPOINT   := service:4822:tool.sd
CAPSULE_REPLY_ENDPOINT     := reply:4823:endpoint.tool.sd.reply
CAPSULE_REQUIRED_CAPS      := 0x19
CAPSULE_CAPS_CEILING       := 0x19
CAPSULE_PREBUILT_BIN       := target/upstream-sd/sd
CAPSULE_METADATA           := crates.io sd v1.0.0 publisher

include nonos-mk/capsule.mk
