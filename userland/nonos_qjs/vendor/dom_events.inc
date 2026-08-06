/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Taking a listener back off, sending an event from script, and the layout
 * numbers a page measures itself with. Included by dom_bindings.c after the
 * statics these build on.
 */

extern int njs_dom_box(void *host, int node, int which);

/* A listener that cannot be removed is a leak with behaviour: a component
 * that unmounts keeps receiving events and acting on a tree it gave up.
 * The slot is cleared rather than compacted so indices stay stable while a
 * dispatch is walking the table. */
static JSValue el_remove_listener(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 2) return JS_UNDEFINED;
    const char *type = JS_ToCString(ctx, v[0]);
    if (!type) return JS_UNDEFINED;
    int node = node_of(ctx, t);
    for (int i = 0; i < g_lcount; i++) {
        if (!g_listeners[i].live || g_listeners[i].node != node) continue;
        if (!type_eq(g_listeners[i].type, type)) continue;
        /* The same function object, not merely one that looks alike: a
         * listener is identified by identity, and two closures built from
         * the same source are different listeners. */
        if (JS_VALUE_GET_PTR(g_listeners[i].fn) != JS_VALUE_GET_PTR(v[1])) continue;
        JS_FreeValue(ctx, g_listeners[i].fn);
        g_listeners[i].fn = JS_UNDEFINED;
        g_listeners[i].live = 0;
        break;
    }
    JS_FreeCString(ctx, type);
    return JS_UNDEFINED;
}

/* A page that raises its own event, which is how components on the same page
 * talk to each other without knowing about one another. */
static JSValue el_dispatch_event(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1 || !JS_IsObject(v[0])) return JS_TRUE;
    JSValue ty = JS_GetPropertyStr(ctx, v[0], "type");
    const char *type = JS_ToCString(ctx, ty);
    JS_FreeValue(ctx, ty);
    if (!type) return JS_TRUE;
    njs_dispatch_event(ctx, node_of(ctx, t), type);
    JS_FreeCString(ctx, type);
    /* The call reports whether the default survived, which is what a caller
     * branches on. */
    return njs_event_default_prevented() ? JS_FALSE : JS_TRUE;
}

/* Layout numbers. A page measures itself to decide what fits, and a zero
 * from every one of these reads as an element with no size, so code that
 * sizes or positions anything takes the wrong branch every time. */
static JSValue get_box(JSContext *ctx, JSValueConst t, int a, JSValueConst *v, int magic) {
    (void)a; (void)v;
    return JS_NewInt32(ctx, njs_dom_box(JS_GetContextOpaque(ctx), node_of(ctx, t), magic));
}
static JSValue el_bounding_rect(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    void *host = JS_GetContextOpaque(ctx);
    int n = node_of(ctx, t);
    int x = njs_dom_box(host, n, 0), y = njs_dom_box(host, n, 1);
    int w = njs_dom_box(host, n, 2), h = njs_dom_box(host, n, 3);
    JSValue r = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, r, "x", JS_NewInt32(ctx, x));
    JS_SetPropertyStr(ctx, r, "y", JS_NewInt32(ctx, y));
    JS_SetPropertyStr(ctx, r, "left", JS_NewInt32(ctx, x));
    JS_SetPropertyStr(ctx, r, "top", JS_NewInt32(ctx, y));
    JS_SetPropertyStr(ctx, r, "width", JS_NewInt32(ctx, w));
    JS_SetPropertyStr(ctx, r, "height", JS_NewInt32(ctx, h));
    JS_SetPropertyStr(ctx, r, "right", JS_NewInt32(ctx, x + w));
    JS_SetPropertyStr(ctx, r, "bottom", JS_NewInt32(ctx, y + h));
    return r;
}

/* Which box number each accessor reads, in the order njs_dom_box expects. */
static const char *const g_box_props[] = {
    "offsetLeft", "offsetTop", "offsetWidth", "offsetHeight",
    "clientWidth", "clientHeight", "scrollWidth", "scrollHeight", 0,
};
static const int g_box_index[] = { 0, 1, 2, 3, 2, 3, 2, 3 };

static void install_event_ext(JSContext *ctx, JSValueConst el) {
    JS_SetPropertyStr(ctx, el, "removeEventListener",
                      JS_NewCFunction(ctx, el_remove_listener, "removeEventListener", 2));
    JS_SetPropertyStr(ctx, el, "dispatchEvent",
                      JS_NewCFunction(ctx, el_dispatch_event, "dispatchEvent", 1));
    JS_SetPropertyStr(ctx, el, "getBoundingClientRect",
                      JS_NewCFunction(ctx, el_bounding_rect, "getBoundingClientRect", 0));
    for (int i = 0; g_box_props[i]; i++) {
        JSAtom a = JS_NewAtom(ctx, g_box_props[i]);
        JSValue g = JS_NewCFunctionMagic(ctx, get_box, g_box_props[i], 0,
                                         JS_CFUNC_getter_magic, g_box_index[i]);
        JS_DefinePropertyGetSet(ctx, el, a, g, JS_UNDEFINED, JS_PROP_C_W_E);
        JS_FreeAtom(ctx, a);
    }
}

extern char *njs_dom_base_url(void *host);
extern char *njs_dom_resolve(void *host, const char *rel);

/* A relative address made absolute against the page, for the scripts that
 * build a link and for history when it is handed a relative path. */
static JSValue js_resolve(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NewString(ctx, "");
    const char *rel = JS_ToCString(ctx, v[0]);
    if (!rel) return JS_NewString(ctx, "");
    char *abs = njs_dom_resolve(JS_GetContextOpaque(ctx), rel);
    JS_FreeCString(ctx, rel);
    if (!abs) return JS_NewString(ctx, "");
    JSValue r = JS_NewString(ctx, abs);
    free(abs);
    return r;
}

/* Where a script asked to go. The capsule reads it after the run and
 * navigates; doing it from inside would tear down the tree the script is
 * still executing against. */
static char g_nav[1024];
static JSValue js_navigate(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_UNDEFINED;
    const char *href = JS_ToCString(ctx, v[0]);
    if (href) {
        char *abs = njs_dom_resolve(JS_GetContextOpaque(ctx), href);
        const char *use = abs ? abs : href;
        int i = 0;
        for (; use[i] && i < (int)sizeof(g_nav) - 1; i++) g_nav[i] = use[i];
        g_nav[i] = 0;
        if (abs) free(abs);
        JS_FreeCString(ctx, href);
    }
    return JS_UNDEFINED;
}

/* The address a script asked to navigate to since the last read, or an empty
 * string. Reading clears it, so one request is acted on once. */
const char *njs_take_navigation(void) {
    if (!g_nav[0]) return 0;
    static char out[1024];
    int i = 0;
    for (; g_nav[i]; i++) out[i] = g_nav[i];
    out[i] = 0;
    g_nav[0] = 0;
    return out;
}

/* The page's own address and the two helpers the prelude builds location
 * from, installed before it runs so location is right from the first line. */
static void install_page_address(JSContext *ctx, JSValueConst global, void *host) {
    char *base = njs_dom_base_url(host);
    JS_SetPropertyStr(ctx, global, "__njs_base", JS_NewString(ctx, base ? base : ""));
    if (base) free(base);
    JS_SetPropertyStr(ctx, global, "__njs_resolve",
                      JS_NewCFunction(ctx, js_resolve, "__njs_resolve", 1));
    JS_SetPropertyStr(ctx, global, "__njs_navigate",
                      JS_NewCFunction(ctx, js_navigate, "__njs_navigate", 1));
}

/* Run the page's due timers. The queue lives in the prelude, so this is a
 * call into it rather than a table here. Returns how many callbacks ran, so
 * the caller knows whether the tree may have changed under it. */
int njs_flush_timers(JSContext *ctx, double now_ms) {
    JS_UpdateStackTop(JS_GetRuntime(ctx));
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue fn = JS_GetPropertyStr(ctx, global, "__njs_flush_timers");
    int ran = 0;
    if (JS_IsFunction(ctx, fn)) {
        JSValue arg = JS_NewFloat64(ctx, now_ms);
        JSValue r = JS_Call(ctx, fn, JS_UNDEFINED, 1, (JSValueConst *)&arg);
        JS_FreeValue(ctx, arg);
        JS_ToInt32(ctx, &ran, r);
        JS_FreeValue(ctx, r);
    }
    JS_FreeValue(ctx, fn);
    JS_FreeValue(ctx, global);
    /* A callback that resolved a promise leaves jobs behind it; settling them
     * here means the tree is final before the caller lays it out. */
    JSRuntime *rt = JS_GetRuntime(ctx);
    JSContext *p;
    while (JS_ExecutePendingJob(rt, &p) > 0) {}
    return ran;
}
