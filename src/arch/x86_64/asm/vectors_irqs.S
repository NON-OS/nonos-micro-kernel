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
/* The external interrupt gates, 32 and up; the exceptions are in vectors.S. */

.intel_syntax noprefix
.text
#include "vec_gate.inc"

VEC_NOERR isr_irq0                 32
VEC_NOERR isr_irq1                 33
VEC_NOERR isr_irq2                 34
VEC_NOERR isr_irq3                 35
VEC_NOERR isr_irq4                 36
VEC_NOERR isr_irq5                 37
VEC_NOERR isr_irq6                 38
VEC_NOERR isr_irq7                 39
VEC_NOERR isr_irq8                 40
VEC_NOERR isr_irq9                 41
VEC_NOERR isr_irq10                42
VEC_NOERR isr_irq11                43
VEC_NOERR isr_irq12                44
VEC_NOERR isr_irq13                45
VEC_NOERR isr_irq14                46
VEC_NOERR isr_irq15                47
VEC_NOERR isr_generic_48           48

/* Inter-processor interrupts. Without these gates the first IPI sent once
 * SMP is enabled faults the target CPU; dispatch routes them through the
 * registered other-handlers and signals LAPIC EOI. */
VEC_NOERR isr_ipi_64               64
VEC_NOERR isr_ipi_65               65
VEC_NOERR isr_ipi_66               66
VEC_NOERR isr_ipi_67               67
VEC_NOERR isr_ipi_68               68

VEC_NOERR isr_syscall             128
