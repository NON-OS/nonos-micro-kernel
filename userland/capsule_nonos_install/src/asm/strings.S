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

# Installer text. Each string is a symbol plus a length symbol so the
# callers never count bytes by hand. The step names line up one to one
# with the step table in entry.S.

.section .rodata

.macro STR name, text:vararg
\name: .ascii \text
.set \name\()_len, . - \name
.endm

STR banner,  "NONOS install\n==============\n"
STR pending, "pending\n"
STR done,    "\ninstall: all steps ran\n"
STR nl,      "\n"

STR t_devcount, "  devices vouched by the broker: 0x"
STR t_dev,      "    dev 0x"
STR t_pci,      "  pci "
STR t_yes,      "yes\n"
STR t_no,       "NO\n"
STR t_f_zk,     "  stark proof verified   "
STR t_f_sig,    "  kernel signature       "
STR t_f_sb,     "  secure boot            "
STR t_f_att,    "  attestation recorded   "
STR t_kroot,    "  kernel blake3  "
STR t_proot,    "\n  program hash   "

STR s1, "survey the machine\n"
STR s2, "compose the system image\n"
STR s3, "read the boot chain verdict\n"
STR s4, "mint the machine attestation root\n"
STR s5, "enroll and sign the composed image\n"
STR s6, "write the boot partition\n"
STR s7, "print the root receipt\n"
