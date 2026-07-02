/*
 * NØNOS Operating System
 * Copyright (C) 2026 NØNOS Contributors
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
 */
#include <stddef.h>
#include <stdint.h>

#define NONOS_MLDSA_ARENA_SIZE (1024u * 1024u)

static uint8_t nonos_mldsa_arena[NONOS_MLDSA_ARENA_SIZE];
static size_t nonos_mldsa_offset;

void *malloc(size_t size) {
    size_t aligned = (size + 15u) & ~15u;
    size_t next = nonos_mldsa_offset + aligned;
    if (next > NONOS_MLDSA_ARENA_SIZE) {
        return NULL;
    }
    void *ptr = &nonos_mldsa_arena[nonos_mldsa_offset];
    nonos_mldsa_offset = next;
    return ptr;
}

void free(void *ptr) {
    (void)ptr;
}

void exit(int status) {
    (void)status;
    for (;;) {
    }
}
