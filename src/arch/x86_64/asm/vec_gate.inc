/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
/* One gate per vector: normalise the stack to vector, error code, iretq
 * frame, and fall through to interrupt_common. See vectors.S.
 */

/* CPU pushes no error code: supply the zero, then the vector. */
.macro VEC_NOERR name vector
.global \name
.p2align 4
\name:
    push    0
    push    \vector
    jmp     interrupt_common
.endm

/* CPU pushes the error code: only the vector is missing. */
.macro VEC_ERR name vector
.global \name
.p2align 4
\name:
    push    \vector
    jmp     interrupt_common
.endm

