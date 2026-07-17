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
size_t strlen(const char *);

static char *dup_cstr(const char *s) {
    if (!s) return 0;
    size_t n = strlen(s) + 1;
    char *r = (char *)malloc(n);
    if (r) for (size_t i = 0; i < n; i++) r[i] = s[i];
    return r;
}

JSRuntime *njs_new_runtime(void) { return JS_NewRuntime(); }
JSContext *njs_new_context(JSRuntime *rt) { return JS_NewContext(rt); }
void njs_free_context(JSContext *ctx) { JS_FreeContext(ctx); }
void njs_free_runtime(JSRuntime *rt) { JS_FreeRuntime(rt); }

/* Evaluate `code` and return a malloc'd string: the result coerced to string,
 * or the exception message. The caller frees it. Pending jobs (resolved
 * promises, queued microtasks) are drained first. */
char *njs_eval_to_string(JSContext *ctx, const char *code, size_t len) {
    JSValue v = JS_Eval(ctx, code, len, "<nonos>", JS_EVAL_TYPE_GLOBAL);
    JSRuntime *rt = JS_GetRuntime(ctx);
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
