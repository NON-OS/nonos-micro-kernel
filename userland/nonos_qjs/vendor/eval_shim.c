/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * A thin C surface over QuickJS whose inline JSValue handling cannot be called
 * from Rust directly. Everything that touches a JSValue stays here; Rust only
 * exchanges runtime/context handles and malloc'd C strings.
 */
#include "quickjs.h"
#include <stddef.h>

void *malloc(size_t);
void *calloc(size_t, size_t);
void free(void *);
void *realloc(void *, size_t);
size_t malloc_usable_size(const void *);
size_t strlen(const char *);

/* Hand QuickJS our allocator explicitly. Its default picks a usable-size
 * function by platform macro, and on this bare target that resolves to a stub
 * returning 0, which breaks the arena allocator's block accounting and corrupts
 * the runtime during JS_NewRuntime. Routing usable-size to our real header read
 * keeps the arena correct. */
static void *mf_calloc(void *o, size_t n, size_t s) { (void)o; return calloc(n, s); }
static void *mf_malloc(void *o, size_t s) { (void)o; return malloc(s); }
static void mf_free(void *o, void *p) { (void)o; free(p); }
static void *mf_realloc(void *o, void *p, size_t s) { (void)o; return realloc(p, s); }
static size_t mf_usable(const void *p) { return malloc_usable_size(p); }

static const JSMallocFunctions nonos_mf = {
    mf_calloc, mf_malloc, mf_free, mf_realloc, mf_usable,
};

static char *dup_cstr(const char *s) {
    if (!s) return 0;
    size_t n = strlen(s) + 1;
    char *r = (char *)malloc(n);
    if (r) for (size_t i = 0; i < n; i++) r[i] = s[i];
    return r;
}

JSRuntime *njs_new_runtime(void) { return JS_NewRuntime2(&nonos_mf, NULL); }
JSContext *njs_new_context(JSRuntime *rt) { return JS_NewContext(rt); }
void njs_free_context(JSContext *ctx) { JS_FreeContext(ctx); }
void njs_free_runtime(JSRuntime *rt) { JS_FreeRuntime(rt); }

/* Evaluate `code` and return a malloc'd string: the result coerced to string,
 * or the exception message. The caller frees it. Pending jobs (resolved
 * promises, queued microtasks) are drained first. */
char *njs_eval_to_string(JSContext *ctx, const char *code, size_t len) {
    JSRuntime *rt = JS_GetRuntime(ctx);
    /* Re-anchor the stack limit to this native frame. The runtime captures its
     * stack top at creation, which may sit far above the frame a page script
     * actually evaluates from (the render loop), so without this a shallow
     * script can trip a false stack-overflow. */
    JS_UpdateStackTop(rt);
    /* JS_Eval requires input[len] == '\0'. The caller passes a Rust &str, which
     * is not NUL-terminated, so copy into a terminated buffer; otherwise the
     * lexer reads past the source and faults on a page boundary. */
    char *src = (char *)malloc(len + 1);
    if (!src) return dup_cstr("out of memory");
    for (size_t i = 0; i < len; i++) src[i] = code[i];
    src[len] = 0;
    JSValue v = JS_Eval(ctx, src, len, "<nonos>", JS_EVAL_TYPE_GLOBAL);
    free(src);
    JSContext *pending;
    while (JS_ExecutePendingJob(rt, &pending) > 0) {}
    char *out;
    if (JS_IsException(v)) {
        JSValue e = JS_GetException(ctx);
        const char *s = JS_ToCString(ctx, e);
        out = dup_cstr(s ? s : "exception");
        JS_FreeCString(ctx, s);
        JS_FreeValue(ctx, e);
    } else {
        const char *s = JS_ToCString(ctx, v);
        out = dup_cstr(s ? s : "");
        JS_FreeCString(ctx, s);
    }
    JS_FreeValue(ctx, v);
    return out;
}
