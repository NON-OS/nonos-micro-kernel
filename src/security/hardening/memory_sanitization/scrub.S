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

# The scrub. A wipe whose loop a compiler wrote is a wipe a compiler may
# reason about, and the promise this routine gives is exactly the one
# dead-store elimination erodes. Here the promise is four instructions.
#
# nonos_scrub_bytes(ptr: rdi, len: rsi), SysV.
#   rep stosb consumes rdi and rcx and stores al, all caller-saved, so the
#   routine clobbers nothing it must preserve. Zero length falls through the
#   rep prefix by the instruction's own definition. sfence orders the stores
#   before whatever the caller does next, poweroff included.

.text
.global nonos_scrub_bytes
.p2align 4
nonos_scrub_bytes:
    mov     rcx, rsi
    xor     eax, eax
    rep stosb
    sfence
    ret
