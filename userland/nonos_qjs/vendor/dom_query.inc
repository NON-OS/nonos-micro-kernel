/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Selector questions about a single node, markup read back out, and the
 * attribute-backed collections a script expects on an element. Included by
 * dom_bindings.c after the statics these build on.
 */

extern int njs_dom_matches(void *host, int node, const char *sel);
extern int njs_dom_closest(void *host, int node, const char *sel);
extern char *njs_dom_get_inner_html(void *host, int node);
extern char *njs_dom_get_outer_html(void *host, int node);
extern int njs_dom_attr_count(void *host, int node);
extern char *njs_dom_attr_name_at(void *host, int node, int i);

static JSValue get_class_list(JSContext *ctx, JSValueConst t, int a, JSValueConst *v);
static JSValue get_dataset(JSContext *ctx, JSValueConst t, int a, JSValueConst *v);

static JSValue el_matches(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_FALSE;
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_FALSE;
    int m = njs_dom_matches(JS_GetContextOpaque(ctx), node_of(ctx, t), s);
    JS_FreeCString(ctx, s);
    return m ? JS_TRUE : JS_FALSE;
}

/* How a page turns a click on whatever was under the pointer into the row or
 * link the handler is about. A page that delegates its events and cannot
 * call this handles nothing at all. */
static JSValue el_closest(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_NULL;
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_NULL;
    int n = njs_dom_closest(JS_GetContextOpaque(ctx), node_of(ctx, t), s);
    JS_FreeCString(ctx, s);
    return make_element(ctx, n);
}

static JSValue get_inner_html(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    char *s = njs_dom_get_inner_html(JS_GetContextOpaque(ctx), node_of(ctx, t));
    if (!s) return JS_NewString(ctx, "");
    JSValue r = JS_NewString(ctx, s);
    free(s);
    return r;
}
static JSValue get_outer_html(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    char *s = njs_dom_get_outer_html(JS_GetContextOpaque(ctx), node_of(ctx, t));
    if (!s) return JS_NewString(ctx, "");
    JSValue r = JS_NewString(ctx, s);
    free(s);
    return r;
}

static JSValue el_get_attribute_names(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    void *host = JS_GetContextOpaque(ctx);
    int n = node_of(ctx, t), count = njs_dom_attr_count(host, n);
    JSValue arr = JS_NewArray(ctx);
    for (int i = 0; i < count; i++) {
        char *k = njs_dom_attr_name_at(host, n, i);
        if (!k) continue;
        JS_SetPropertyUint32(ctx, arr, (uint32_t)i, JS_NewString(ctx, k));
        free(k);
    }
    return arr;
}

/* Descendants by tag or class. Both are the selector engine with the name
 * spelled as a selector, which is what the browser does with them anyway. */
static JSValue collect_sel(JSContext *ctx, JSValueConst t, const char *sel) {
    void *host = JS_GetContextOpaque(ctx);
    int root = node_of(ctx, t);
    int buf[512];
    int n = njs_dom_query_all(host, sel, buf, 512);
    JSValue arr = JS_NewArray(ctx);
    uint32_t at = 0;
    for (int i = 0; i < n; i++)
        if (buf[i] != root && njs_dom_contains(host, root, buf[i]))
            JS_SetPropertyUint32(ctx, arr, at++, make_element(ctx, buf[i]));
    return arr;
}
static JSValue el_by_tag(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_NewArray(ctx);
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_NewArray(ctx);
    JSValue r = collect_sel(ctx, t, s);
    JS_FreeCString(ctx, s);
    return r;
}
static JSValue el_by_class(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_NewArray(ctx);
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_NewArray(ctx);
    char sel[128];
    sel[0] = '.';
    int i = 0;
    for (; s[i] && i < 126; i++) sel[i + 1] = s[i] == ' ' ? '.' : s[i];
    sel[i + 1] = 0;
    JS_FreeCString(ctx, s);
    return collect_sel(ctx, t, sel);
}

static void install_query_ext(JSContext *ctx, JSValueConst el) {
    JS_SetPropertyStr(ctx, el, "matches", JS_NewCFunction(ctx, el_matches, "matches", 1));
    JS_SetPropertyStr(ctx, el, "closest", JS_NewCFunction(ctx, el_closest, "closest", 1));
    JS_SetPropertyStr(ctx, el, "getAttributeNames",
                      JS_NewCFunction(ctx, el_get_attribute_names, "getAttributeNames", 0));
    JS_SetPropertyStr(ctx, el, "getElementsByTagName",
                      JS_NewCFunction(ctx, el_by_tag, "getElementsByTagName", 1));
    JS_SetPropertyStr(ctx, el, "getElementsByClassName",
                      JS_NewCFunction(ctx, el_by_class, "getElementsByClassName", 1));
    accessor(ctx, el, "outerHTML", get_outer_html, 0);
    accessor(ctx, el, "classList", get_class_list, 0);
    accessor(ctx, el, "dataset", get_dataset, 0);
}

/* classList and dataset are built in JS from the element's own attribute
 * methods, because both are small and neither needs the host. The factories
 * live in the prelude; these hand the element to them.
 *
 * They are built per read rather than cached, which is what a script that
 * writes a class and reads it back in the same statement needs. */
static JSValue from_factory(JSContext *ctx, JSValueConst t, const char *factory) {
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue fn = JS_GetPropertyStr(ctx, global, factory);
    JSValue r = JS_UNDEFINED;
    if (JS_IsFunction(ctx, fn)) {
        JSValueConst arg = t;
        r = JS_Call(ctx, fn, JS_UNDEFINED, 1, &arg);
    }
    JS_FreeValue(ctx, fn);
    JS_FreeValue(ctx, global);
    return r;
}
static JSValue get_class_list(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return from_factory(ctx, t, "__njs_classlist");
}
static JSValue get_dataset(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return from_factory(ctx, t, "__njs_dataset");
}

/* The document-wide forms. These search the whole tree rather than one
 * element's descendants, so they cannot share the element versions. */
static JSValue doc_by_tag(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NewArray(ctx);
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_NewArray(ctx);
    int buf[512];
    int n = njs_dom_query_all(JS_GetContextOpaque(ctx), s, buf, 512);
    JS_FreeCString(ctx, s);
    JSValue arr = JS_NewArray(ctx);
    for (int i = 0; i < n; i++)
        JS_SetPropertyUint32(ctx, arr, (uint32_t)i, make_element(ctx, buf[i]));
    return arr;
}
static JSValue doc_by_class(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NewArray(ctx);
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_NewArray(ctx);
    char sel[128];
    sel[0] = '.';
    int i = 0;
    for (; s[i] && i < 126; i++) sel[i + 1] = s[i] == ' ' ? '.' : s[i];
    sel[i + 1] = 0;
    JS_FreeCString(ctx, s);
    int buf[512];
    int n = njs_dom_query_all(JS_GetContextOpaque(ctx), sel, buf, 512);
    JSValue arr = JS_NewArray(ctx);
    for (int k = 0; k < n; k++)
        JS_SetPropertyUint32(ctx, arr, (uint32_t)k, make_element(ctx, buf[k]));
    return arr;
}
