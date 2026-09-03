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

# Install steps whose kernel service has not landed yet. Every handler
# follows one contract: no arguments, rax nonzero on refusal, all other
# registers preserved by the caller. Survey and boot-verdict live in
# their own files; the ones below announce themselves pending and
# succeed, so the ritual's frame runs end to end while their services
# are built.

.text

.macro STEP name
.global \name
.p2align 4
\name:
    lea     rdi, [rip + pending]
    mov     esi, dword ptr [rip + pending_len_w]
    call    console_write
    xor     eax, eax
    ret
.endm

STEP step_mint_root
STEP step_enroll
STEP step_write_esp
STEP step_receipt
