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

# Syscall tags, the four byte codes the kernel dispatches on, little endian
# in rax. The values mirror src/syscall/numbers in the kernel; a change on
# either side is caught by the ABI registry, not by luck.
#
# Convention for every call: rax the tag, arguments in rdi rsi rdx r10 r8 r9,
# result in rax, negative on refusal. Nothing survives a syscall but rax.

.set SYS_STDOUT,       0x574F534D     # "MSOW" console write: ptr, len
.set SYS_DEBUG,        0x4742444D     # "MDBG" serial trace:  ptr, len
.set SYS_EXIT,         0x5458454D     # "MEXT" terminate:     code
.set SYS_YIELD,        0x444C594D     # "MYLD"
.set SYS_CAPSULE_VERIFY,0x4656434D    # "MCVF" capsule verify: req, out
.set SYS_DEVICE_LIST,  0x534C444D     # "MDLS" enumerate: class, buf, count
.set SYS_ATTEST_STATUS,0x5453414D     # "MAST" boot attestation: out
.set SYS_SERVICE_LOOKUP,0x4C56534D    # "MSVL" resolve: name, len, port, pid
.set SYS_IPC_CALL,     0x4C43494D     # "MICL" call: ep, req, len, resp, cap, ms
