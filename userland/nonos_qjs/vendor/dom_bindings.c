/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * The QuickJS <-> DOM glue. A DOM element is a JS object carrying a hidden
 * __node integer (its index in the host DOM). Mutating methods and property
 * accessors route through njs_dom_* callbacks the host implements over its real
 * node tree. The host pointer travels as the context opaque.
 */
#include "quickjs.h"

void free(void *);

extern int njs_dom_create_element(void *host, const char *tag);
extern void njs_dom_append(void *host, int parent, int child);
extern void njs_dom_remove_child(void *host, int parent, int child);
extern void njs_dom_set_attr(void *host, int node, const char *k, const char *v);
extern char *njs_dom_get_attr(void *host, int node, const char *k);
extern void njs_dom_set_text(void *host, int node, const char *text);
extern char *njs_dom_get_text(void *host, int node);
extern char *njs_dom_get_tag(void *host, int node);
extern int njs_dom_get_by_id(void *host, const char *id);
extern int njs_dom_query(void *host, const char *sel);
extern int njs_dom_query_all(void *host, const char *sel, int *out, int max);
extern int njs_dom_body(void *host);
extern void njs_dom_set_style(void *host, int node, const char *prop, const char *val);
extern void njs_dom_set_inner_html(void *host, int node, const char *html);

/* Event listeners are held here with a duplicated function reference so the GC
 * keeps them alive between the script run and later dispatch. One table per
 * process is correct: a capsule runs one page context. */
#define MAX_LISTENERS 1024
static struct {
    int node;
    char type[24];
    JSValue fn;
    int live;
} g_listeners[MAX_LISTENERS];
static int g_lcount = 0;

static int node_of(JSContext *ctx, JSValueConst obj) {
    JSValue v = JS_GetPropertyStr(ctx, obj, "__node");
    int n = -1;
    JS_ToInt32(ctx, &n, v);
    JS_FreeValue(ctx, v);
    return n;
}

static JSValue str_or_null(JSContext *ctx, char *s) {
    if (!s) return JS_NULL;
    JSValue v = JS_NewString(ctx, s);
    free(s);
    return v;
}

static JSValue make_element(JSContext *ctx, int node);

static JSValue el_append_child(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    njs_dom_append(JS_GetContextOpaque(ctx), node_of(ctx, t), node_of(ctx, v[0]));
    return JS_DupValue(ctx, v[0]);
}

static JSValue el_remove_child(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    njs_dom_remove_child(JS_GetContextOpaque(ctx), node_of(ctx, t), node_of(ctx, v[0]));
    return JS_DupValue(ctx, v[0]);
}

static JSValue el_set_attribute(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 2) return JS_UNDEFINED;
    const char *k = JS_ToCString(ctx, v[0]);
    const char *val = JS_ToCString(ctx, v[1]);
    njs_dom_set_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), k, val);
    JS_FreeCString(ctx, k);
    JS_FreeCString(ctx, val);
    return JS_UNDEFINED;
}

static JSValue el_get_attribute(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_NULL;
    const char *k = JS_ToCString(ctx, v[0]);
    char *r = njs_dom_get_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), k);
    JS_FreeCString(ctx, k);
    return str_or_null(ctx, r);
}

static JSValue set_text_content(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    const char *s = JS_ToCString(ctx, v[0]);
    njs_dom_set_text(JS_GetContextOpaque(ctx), node_of(ctx, t), s);
    JS_FreeCString(ctx, s);
    return JS_UNDEFINED;
}
static JSValue get_text_content(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return str_or_null(ctx, njs_dom_get_text(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue set_class_name(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    const char *s = JS_ToCString(ctx, v[0]);
    njs_dom_set_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), "class", s);
    JS_FreeCString(ctx, s);
    return JS_UNDEFINED;
}
static JSValue get_class_name(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return str_or_null(ctx, njs_dom_get_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), "class"));
}
static JSValue set_id(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    const char *s = JS_ToCString(ctx, v[0]);
    njs_dom_set_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), "id", s);
    JS_FreeCString(ctx, s);
    return JS_UNDEFINED;
}
static JSValue get_tag_name(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return str_or_null(ctx, njs_dom_get_tag(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue set_inner_html(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    const char *h = JS_ToCString(ctx, v[0]);
    njs_dom_set_inner_html(JS_GetContextOpaque(ctx), node_of(ctx, t), h);
    JS_FreeCString(ctx, h);
    return JS_UNDEFINED;
}

static JSValue el_add_listener(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 2) return JS_UNDEFINED;
    const char *type = JS_ToCString(ctx, v[0]);
    if (g_lcount < MAX_LISTENERS) {
        int i = g_lcount++;
        g_listeners[i].node = node_of(ctx, t);
        int k = 0;
        while (type[k] && k < 23) {
            g_listeners[i].type[k] = type[k];
            k++;
        }
        g_listeners[i].type[k] = 0;
        g_listeners[i].fn = JS_DupValue(ctx, v[1]);
        g_listeners[i].live = 1;
    }
    JS_FreeCString(ctx, type);
    return JS_UNDEFINED;
}

static int type_eq(const char *a, const char *b) {
    int i = 0;
    while (a[i] && a[i] == b[i]) i++;
    return a[i] == 0 && b[i] == 0;
}

/* Dispatch a UI event to a node's registered listeners. Callable from the host
 * (Rust) when a real pointer/key event lands on a laid-out node. Drains the job
 * queue so listener-scheduled microtasks settle before layout. */
int njs_dispatch_event(JSContext *ctx, int node, const char *type) {
    JS_UpdateStackTop(JS_GetRuntime(ctx));
    int fired = 0;
    JSValue ev = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, ev, "type", JS_NewString(ctx, type));
    JS_SetPropertyStr(ctx, ev, "target", make_element(ctx, node));
    for (int i = 0; i < g_lcount; i++) {
        if (g_listeners[i].live && g_listeners[i].node == node && type_eq(g_listeners[i].type, type)) {
            JSValue arg = ev;
            JSValue r = JS_Call(ctx, g_listeners[i].fn, JS_UNDEFINED, 1, (JSValueConst *)&arg);
            JS_FreeValue(ctx, r);
            fired++;
        }
    }
    JS_FreeValue(ctx, ev);
    JSRuntime *rt = JS_GetRuntime(ctx);
    JSContext *p;
    while (JS_ExecutePendingJob(rt, &p) > 0) {}
    return fired;
}

static void accessor(JSContext *ctx, JSValueConst el, const char *name, JSCFunction *get, JSCFunction *set) {
    JSAtom a = JS_NewAtom(ctx, name);
    JSValue g = get ? JS_NewCFunction(ctx, get, name, 0) : JS_UNDEFINED;
    JSValue s = set ? JS_NewCFunction(ctx, set, name, 1) : JS_UNDEFINED;
    JS_DefinePropertyGetSet(ctx, el, a, g, s, JS_PROP_C_W_E);
    JS_FreeAtom(ctx, a);
}

/* el.style.<prop> = <val>: a Proxy whose target carries the node, so the set
 * trap knows which element to style. Individual property assignment is what
 * frameworks emit; it accumulates into the node's inline style. */
static JSValue style_set_trap(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 3) return JS_TRUE;
    const char *prop = JS_ToCString(ctx, v[1]);
    const char *val = JS_ToCString(ctx, v[2]);
    njs_dom_set_style(JS_GetContextOpaque(ctx), node_of(ctx, v[0]), prop, val);
    JS_FreeCString(ctx, prop);
    JS_FreeCString(ctx, val);
    return JS_TRUE;
}

static JSValue get_style(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)argc; (void)v;
    JSValue target = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, target, "__node", JS_NewInt32(ctx, node_of(ctx, t)));
    JSValue handler = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, handler, "set", JS_NewCFunction(ctx, style_set_trap, "set", 4));
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue proxy_ctor = JS_GetPropertyStr(ctx, global, "Proxy");
    JSValue args[2] = {target, handler};
    JSValue proxy = JS_CallConstructor(ctx, proxy_ctor, 2, args);
    JS_FreeValue(ctx, proxy_ctor);
    JS_FreeValue(ctx, global);
    JS_FreeValue(ctx, target);
    JS_FreeValue(ctx, handler);
    return proxy;
}

static JSValue make_element(JSContext *ctx, int node) {
    if (node < 0) return JS_NULL;
    JSValue el = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, el, "__node", JS_NewInt32(ctx, node));
    JS_SetPropertyStr(ctx, el, "appendChild", JS_NewCFunction(ctx, el_append_child, "appendChild", 1));
    JS_SetPropertyStr(ctx, el, "removeChild", JS_NewCFunction(ctx, el_remove_child, "removeChild", 1));
    JS_SetPropertyStr(ctx, el, "setAttribute", JS_NewCFunction(ctx, el_set_attribute, "setAttribute", 2));
    JS_SetPropertyStr(ctx, el, "getAttribute", JS_NewCFunction(ctx, el_get_attribute, "getAttribute", 1));
    JS_SetPropertyStr(ctx, el, "addEventListener", JS_NewCFunction(ctx, el_add_listener, "addEventListener", 2));
    accessor(ctx, el, "textContent", get_text_content, set_text_content);
    accessor(ctx, el, "className", get_class_name, set_class_name);
    accessor(ctx, el, "id", 0, set_id);
    accessor(ctx, el, "tagName", get_tag_name, 0);
    accessor(ctx, el, "style", get_style, 0);
    accessor(ctx, el, "innerHTML", 0, set_inner_html);
    return el;
}

static JSValue doc_create_element(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NULL;
    const char *tag = JS_ToCString(ctx, v[0]);
    int node = njs_dom_create_element(JS_GetContextOpaque(ctx), tag);
    JS_FreeCString(ctx, tag);
    return make_element(ctx, node);
}
static JSValue doc_get_by_id(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NULL;
    const char *id = JS_ToCString(ctx, v[0]);
    int node = njs_dom_get_by_id(JS_GetContextOpaque(ctx), id);
    JS_FreeCString(ctx, id);
    return make_element(ctx, node);
}
static JSValue doc_query(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NULL;
    const char *sel = JS_ToCString(ctx, v[0]);
    int node = njs_dom_query(JS_GetContextOpaque(ctx), sel);
    JS_FreeCString(ctx, sel);
    return make_element(ctx, node);
}
static JSValue doc_query_all(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    JSValue arr = JS_NewArray(ctx);
    if (argc < 1) return arr;
    const char *sel = JS_ToCString(ctx, v[0]);
    int ids[256];
    int n = njs_dom_query_all(JS_GetContextOpaque(ctx), sel, ids, 256);
    JS_FreeCString(ctx, sel);
    for (int i = 0; i < n; i++)
        JS_SetPropertyUint32(ctx, arr, i, make_element(ctx, ids[i]));
    return arr;
}

void njs_install_dom(JSContext *ctx, void *host) {
    JS_SetContextOpaque(ctx, host);
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue doc = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, doc, "createElement", JS_NewCFunction(ctx, doc_create_element, "createElement", 1));
    JS_SetPropertyStr(ctx, doc, "getElementById", JS_NewCFunction(ctx, doc_get_by_id, "getElementById", 1));
    JS_SetPropertyStr(ctx, doc, "querySelector", JS_NewCFunction(ctx, doc_query, "querySelector", 1));
    JS_SetPropertyStr(ctx, doc, "querySelectorAll", JS_NewCFunction(ctx, doc_query_all, "querySelectorAll", 1));
    JS_SetPropertyStr(ctx, doc, "body", make_element(ctx, njs_dom_body(host)));
    JS_SetPropertyStr(ctx, global, "document", doc);
    JS_FreeValue(ctx, global);
}
