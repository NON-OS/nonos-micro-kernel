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
extern int njs_dom_create_text(void *host, const char *text);
extern void njs_dom_insert_before(void *host, int parent, int child, int before);
extern int njs_dom_parent(void *host, int node);
extern int njs_dom_child_count(void *host, int node);
extern int njs_dom_child_at(void *host, int node, int i);
extern int njs_dom_next_sibling(void *host, int node);
extern int njs_dom_node_kind(void *host, int node);
extern void njs_dom_remove_attr(void *host, int node, const char *k);

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

/* insertBefore(child, ref); a null ref appends, per the DOM contract. */
static JSValue el_insert_before(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    int before = (argc >= 2 && JS_IsObject(v[1])) ? node_of(ctx, v[1]) : -1;
    njs_dom_insert_before(JS_GetContextOpaque(ctx), node_of(ctx, t), node_of(ctx, v[0]), before);
    return JS_DupValue(ctx, v[0]);
}

static JSValue el_remove_attribute(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    const char *k = JS_ToCString(ctx, v[0]);
    njs_dom_remove_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), k);
    JS_FreeCString(ctx, k);
    return JS_UNDEFINED;
}

static JSValue el_remove(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)argc; (void)v;
    int n = node_of(ctx, t);
    njs_dom_remove_child(JS_GetContextOpaque(ctx), njs_dom_parent(JS_GetContextOpaque(ctx), n), n);
    return JS_UNDEFINED;
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

/* Event types compare case-insensitively: frameworks that probe for
 * on<type> host properties before lowercasing register mixed-case names. */
static int type_eq(const char *a, const char *b) {
    int i = 0;
    for (;; i++) {
        char x = a[i], y = b[i];
        if (x >= 'A' && x <= 'Z') x += 32;
        if (y >= 'A' && y <= 'Z') y += 32;
        if (x != y) return 0;
        if (x == 0) return 1;
    }
}

/* An event's own methods have to record something. These were no-ops, so a
 * handler that cancelled a link still navigated and one that stopped
 * propagation still saw the event reach every ancestor above it. Both are
 * how an interactive page keeps control of what it just handled. */
static JSValue ev_prevent(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)argc; (void)v;
    JSValue c = JS_GetPropertyStr(ctx, t, "cancelable");
    if (JS_ToBool(ctx, c) > 0)
        JS_SetPropertyStr(ctx, t, "defaultPrevented", JS_TRUE);
    JS_FreeValue(ctx, c);
    return JS_UNDEFINED;
}
static JSValue ev_stop(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)argc; (void)v;
    JS_SetPropertyStr(ctx, t, "__stop", JS_TRUE);
    return JS_UNDEFINED;
}
static JSValue ev_stop_now(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)argc; (void)v;
    JS_SetPropertyStr(ctx, t, "__stop", JS_TRUE);
    JS_SetPropertyStr(ctx, t, "__stopnow", JS_TRUE);
    return JS_UNDEFINED;
}
static int ev_flag(JSContext *ctx, JSValueConst ev, const char *name) {
    JSValue v = JS_GetPropertyStr(ctx, ev, name);
    int set = JS_ToBool(ctx, v) > 0;
    JS_FreeValue(ctx, v);
    return set;
}

/* Whether the last dispatch was cancelled, so the host can skip the action
 * the event would otherwise have triggered. */
static int g_last_prevented = 0;
int njs_event_default_prevented(void) { return g_last_prevented; }

/* Dispatch a UI event to a node's listeners, then bubble through its
 * ancestors: frameworks that delegate (React listens on the root container)
 * see the event with target still pointing at the origin node. Callable from
 * the host when a real pointer/key event lands on a laid-out node. Drains the
 * job queue so listener-scheduled microtasks settle before layout. */
int njs_dispatch_event(JSContext *ctx, int node, const char *type) {
    JS_UpdateStackTop(JS_GetRuntime(ctx));
    void *host = JS_GetContextOpaque(ctx);
    int fired = 0;
    JSValue ev = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, ev, "type", JS_NewString(ctx, type));
    JS_SetPropertyStr(ctx, ev, "target", make_element(ctx, node));
    JS_SetPropertyStr(ctx, ev, "bubbles", JS_TRUE);
    JS_SetPropertyStr(ctx, ev, "cancelable", JS_TRUE);
    JS_SetPropertyStr(ctx, ev, "defaultPrevented", JS_FALSE);
    JS_SetPropertyStr(ctx, ev, "isTrusted", JS_TRUE);
    JS_SetPropertyStr(ctx, ev, "timeStamp", JS_NewInt32(ctx, 0));
    JS_SetPropertyStr(ctx, ev, "preventDefault", JS_NewCFunction(ctx, ev_prevent, "preventDefault", 0));
    JS_SetPropertyStr(ctx, ev, "stopPropagation", JS_NewCFunction(ctx, ev_stop, "stopPropagation", 0));
    JS_SetPropertyStr(ctx, ev, "stopImmediatePropagation", JS_NewCFunction(ctx, ev_stop_now, "stopImmediatePropagation", 0));
    int hops = 0;
    for (int cur = node; cur >= 0 && hops < 512; cur = njs_dom_parent(host, cur), hops++) {
        JSValue self = make_element(ctx, cur);
        JS_SetPropertyStr(ctx, ev, "currentTarget", JS_DupValue(ctx, self));
        /* The on<type> property is a listener too. It was set to null on
         * every element and then never read, so `el.onclick = fn` looked
         * like it had registered and nothing ever called it. */
        char prop[32];
        prop[0] = 'o'; prop[1] = 'n';
        int pi = 0;
        for (; type[pi] && pi < 28; pi++) prop[pi + 2] = type[pi];
        prop[pi + 2] = 0;
        JSValue handler = JS_GetPropertyStr(ctx, self, prop);
        if (JS_IsFunction(ctx, handler)) {
            JSValue arg = ev;
            JSValue r = JS_Call(ctx, handler, self, 1, (JSValueConst *)&arg);
            JS_FreeValue(ctx, r);
            fired++;
        }
        JS_FreeValue(ctx, handler);
        for (int i = 0; i < g_lcount && !ev_flag(ctx, ev, "__stopnow"); i++) {
            if (g_listeners[i].live && g_listeners[i].node == cur && type_eq(g_listeners[i].type, type)) {
                /* Listener `this` is the element it registered on; delegated
                 * handlers (Preact's eventProxy) read their table off it. */
                JSValue arg = ev;
                JSValue r = JS_Call(ctx, g_listeners[i].fn, self, 1, (JSValueConst *)&arg);
                JS_FreeValue(ctx, r);
                fired++;
            }
        }
        JS_FreeValue(ctx, self);
        /* A handler that stopped propagation meant it: the ancestors above
         * this one do not see the event at all. */
        if (ev_flag(ctx, ev, "__stop")) break;
        if (cur == 0) break;
    }
    /* Document-level listeners registered through document.addEventListener
     * carry node -1 and fire last, as the outermost bubble stop. */
    for (int i = 0; i < g_lcount && !ev_flag(ctx, ev, "__stop"); i++) {
        if (g_listeners[i].live && g_listeners[i].node == -1 && type_eq(g_listeners[i].type, type)) {
            JSValue arg = ev;
            JSValue r = JS_Call(ctx, g_listeners[i].fn, JS_UNDEFINED, 1, (JSValueConst *)&arg);
            JS_FreeValue(ctx, r);
            fired++;
        }
    }
    g_last_prevented = ev_flag(ctx, ev, "defaultPrevented");
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
    JS_FreeValue(ctx, target);
    JS_FreeValue(ctx, handler);
    return proxy;
}

static JSValue get_parent_node(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return make_element(ctx, njs_dom_parent(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue get_first_child(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return make_element(ctx, njs_dom_child_at(JS_GetContextOpaque(ctx), node_of(ctx, t), 0));
}
static JSValue get_next_sibling(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return make_element(ctx, njs_dom_next_sibling(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue get_node_type(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return JS_NewInt32(ctx, njs_dom_node_kind(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue get_local_name(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return str_or_null(ctx, njs_dom_get_tag(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
/* nodeName: elements report their tag uppercased, text nodes #text. */
static JSValue get_node_name(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    void *host = JS_GetContextOpaque(ctx);
    int n = node_of(ctx, t);
    if (njs_dom_node_kind(host, n) == 3) return JS_NewString(ctx, "#text");
    char *s = njs_dom_get_tag(host, n);
    if (!s) return JS_NULL;
    for (int i = 0; s[i]; i++)
        if (s[i] >= 'a' && s[i] <= 'z') s[i] -= 32;
    JSValue r = JS_NewString(ctx, s);
    free(s);
    return r;
}
/* ownerDocument hands back the shared document object; event delegation
 * roots (React) listen there. */
static JSValue get_owner_document(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)t; (void)a; (void)v;
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue doc = JS_GetPropertyStr(ctx, global, "document");
    return doc;
}
static JSValue get_child_nodes(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    void *host = JS_GetContextOpaque(ctx);
    int n = node_of(ctx, t);
    JSValue arr = JS_NewArray(ctx);
    int count = njs_dom_child_count(host, n);
    for (int i = 0; i < count; i++)
        JS_SetPropertyUint32(ctx, arr, (uint32_t)i, make_element(ctx, njs_dom_child_at(host, n, i)));
    return arr;
}

/* One JS object per DOM node, cached in a global registry. Frameworks hang
 * expando state off DOM nodes and compare them by identity, so every lookup
 * for the same node must return the same object. */
#include "dom_ext.inc"
#include "dom_query.inc"
#include "dom_events.inc"

static JSValue make_element(JSContext *ctx, int node) {
    if (node < 0) return JS_NULL;
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue reg = JS_GetPropertyStr(ctx, global, "__njs_nodes");
    if (!JS_IsObject(reg)) {
        JS_FreeValue(ctx, reg);
        reg = JS_NewObject(ctx);
        JS_SetPropertyStr(ctx, global, "__njs_nodes", JS_DupValue(ctx, reg));
    }
    JSValue cached = JS_GetPropertyUint32(ctx, reg, (uint32_t)node);
    if (JS_IsObject(cached)) {
        JS_FreeValue(ctx, reg);
        JS_FreeValue(ctx, global);
        return cached;
    }
    JS_FreeValue(ctx, cached);
    JSValue el = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, el, "__node", JS_NewInt32(ctx, node));
    /* Elements carry the on<event> IDL properties as null: frameworks probe
     * `'onclick' in dom` to decide event-name normalization. */
    static const char *onprops[] = {
        "onclick", "ondblclick", "oninput", "onchange", "onsubmit", "onkeydown", "onkeyup",
        "onkeypress", "onmousedown", "onmouseup", "onmousemove", "onmouseover", "onmouseout",
        "onmouseenter", "onmouseleave", "onfocus", "onblur", "onscroll", "ontouchstart",
        "ontouchend", "ontouchmove", "onpointerdown", "onpointerup", "onpointermove", 0,
    };
    for (int i = 0; onprops[i]; i++)
        JS_SetPropertyStr(ctx, el, onprops[i], JS_NULL);
    JS_SetPropertyStr(ctx, el, "appendChild", JS_NewCFunction(ctx, el_append_child, "appendChild", 1));
    JS_SetPropertyStr(ctx, el, "insertBefore", JS_NewCFunction(ctx, el_insert_before, "insertBefore", 2));
    JS_SetPropertyStr(ctx, el, "removeChild", JS_NewCFunction(ctx, el_remove_child, "removeChild", 1));
    JS_SetPropertyStr(ctx, el, "remove", JS_NewCFunction(ctx, el_remove, "remove", 0));
    JS_SetPropertyStr(ctx, el, "setAttribute", JS_NewCFunction(ctx, el_set_attribute, "setAttribute", 2));
    JS_SetPropertyStr(ctx, el, "getAttribute", JS_NewCFunction(ctx, el_get_attribute, "getAttribute", 1));
    JS_SetPropertyStr(ctx, el, "removeAttribute", JS_NewCFunction(ctx, el_remove_attribute, "removeAttribute", 1));
    JS_SetPropertyStr(ctx, el, "addEventListener", JS_NewCFunction(ctx, el_add_listener, "addEventListener", 2));
    accessor(ctx, el, "textContent", get_text_content, set_text_content);
    accessor(ctx, el, "data", get_text_content, set_text_content);
    accessor(ctx, el, "nodeValue", get_text_content, set_text_content);
    accessor(ctx, el, "className", get_class_name, set_class_name);
    accessor(ctx, el, "tagName", get_tag_name, 0);
    accessor(ctx, el, "localName", get_local_name, 0);
    accessor(ctx, el, "nodeName", get_node_name, 0);
    accessor(ctx, el, "nodeType", get_node_type, 0);
    accessor(ctx, el, "parentNode", get_parent_node, 0);
    accessor(ctx, el, "ownerDocument", get_owner_document, 0);
    accessor(ctx, el, "firstChild", get_first_child, 0);
    accessor(ctx, el, "nextSibling", get_next_sibling, 0);
    accessor(ctx, el, "childNodes", get_child_nodes, 0);
    accessor(ctx, el, "style", get_style, 0);
    accessor(ctx, el, "innerHTML", get_inner_html, set_inner_html);
    install_element_ext(ctx, el);
    install_query_ext(ctx, el);
    install_event_ext(ctx, el);
    JS_SetPropertyUint32(ctx, reg, (uint32_t)node, JS_DupValue(ctx, el));
    JS_FreeValue(ctx, reg);
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

static JSValue doc_create_text(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NULL;
    const char *s = JS_ToCString(ctx, v[0]);
    int node = njs_dom_create_text(JS_GetContextOpaque(ctx), s);
    JS_FreeCString(ctx, s);
    return make_element(ctx, node);
}

/* The window shims frameworks probe for before rendering: timer queues that
 * flush on demand, a quiet console, and enough of navigator/location to pass
 * feature checks. Kept in JS so host and capsule share one definition. */
static const char *PRELUDE =
    "(function(){var q=[];"
    "globalThis.window=globalThis;globalThis.self=globalThis;"
    "var sink=function(){};"
    "globalThis.console={log:sink,warn:sink,error:sink,info:sink,debug:sink,trace:sink};"
    /* Timers keep their delay and fire in due order. Dropping the delay ran
     * a chain of deferred steps out of sequence, which is worse than not
     * running it. Cancelling has to work too: a component that unmounts and
     * clears its timer would otherwise still fire into a tree it gave up. */
    "var tid=1,clock=0;"
    "globalThis.setTimeout=function(f,ms){q.push({id:tid,at:clock+(+ms||0),f:f,r:0});return tid++;};"
    "globalThis.setInterval=function(f,ms){var d=Math.max(1,+ms||1);"
    "q.push({id:tid,at:clock+d,f:f,r:d});return tid++;};"
    "globalThis.clearTimeout=function(i){for(var k=0;k<q.length;k++)"
    "if(q[k].id===i){q.splice(k,1);return;}};"
    "globalThis.clearInterval=globalThis.clearTimeout;"
    "globalThis.requestAnimationFrame=function(f){q.push({id:tid,at:clock,f:f,r:0});return tid++;};"
    "globalThis.cancelAnimationFrame=globalThis.clearTimeout;"
    "globalThis.queueMicrotask=globalThis.queueMicrotask||function(f){Promise.resolve().then(f);};"
    /* Due timers first, then the clock moves to the next one waiting. A
     * repeating timer is requeued rather than dropped, and the run count is
     * capped so a timer that reschedules itself cannot hold the pump. */
    /* Run what the clock says is due, and nothing else. The caller passes
     * real elapsed milliseconds, so a repeating timer requeued during this
     * flush lands in the future and waits there. Advancing the clock to meet
     * the earliest timer instead would make every requeue instantly due, and
     * a single setInterval would then run its callback until the iteration
     * cap on every tick, which is a page that never stops working long
     * enough to draw. */
    "globalThis.__njs_flush_timers=function(now){var n=0;"
    "if(now>clock)clock=+now;"
    "while(q.length&&n<1000){"
    "var i=0;for(var k=1;k<q.length;k++)if(q[k].at<q[i].at)i=k;"
    "var t=q[i];if(t.at>clock)break;q.splice(i,1);n++;"
    "if(t.r){q.push({id:t.id,at:clock+t.r,f:t.f,r:t.r});}"
    "try{t.f();}catch(e){"
    "(globalThis.__njs_errors=globalThis.__njs_errors||[]).push(String(e)+' @ '+String(e&&e.stack||''));}}"
    "return n;};"
    "globalThis.navigator={userAgent:'NONOS Browser',language:'en-US',languages:['en-US']};"
    /* location is built from the address the page was actually fetched from.
     * It used to say http://localhost/ whatever had been loaded, so a page
     * that reads its own path to decide what to show, which is every page
     * with a router in it, was told something that was never true. */
    "globalThis.__njs_mkloc=function(u){"
    "var m=/^([a-z]+:)\\/\\/([^\\/?#]*)([^?#]*)(\\?[^#]*)?(#.*)?$/i.exec(u||'')||[];"
    "var proto=m[1]||'http:',hostport=m[2]||'',path=m[3]||'/';"
    "var at=hostport.lastIndexOf(':');"
    "var hasport=at>0&&/^[0-9]+$/.test(hostport.slice(at+1));"
    "return{href:u||'',protocol:proto,host:hostport,"
    "hostname:hasport?hostport.slice(0,at):hostport,"
    "port:hasport?hostport.slice(at+1):'',"
    "pathname:path||'/',search:m[4]||'',hash:m[5]||'',"
    "origin:proto+'//'+hostport,"
    "toString:function(){return this.href;},"
    "assign:function(h){globalThis.__njs_navigate(String(h));},"
    "replace:function(h){globalThis.__njs_navigate(String(h));},"
    "reload:function(){globalThis.__njs_navigate(this.href);}};};"
    "globalThis.location=globalThis.__njs_mkloc(globalThis.__njs_base||'');"
    /* A page that pushes state is telling the reader it moved. The address
     * has to follow, or the next relative link resolves against the old one
     * and code reading the path after a route change sees the previous one. */
    "globalThis.history={length:1,state:null,scrollRestoration:'auto',"
    "pushState:function(s,t,u){this.state=s;if(u)globalThis.location="
    "globalThis.__njs_mkloc(globalThis.__njs_resolve(String(u)));this.length++;},"
    "replaceState:function(s,t,u){this.state=s;if(u)globalThis.location="
    "globalThis.__njs_mkloc(globalThis.__njs_resolve(String(u)));},"
    "back:sink,forward:sink,go:sink};"
    "if(globalThis.document){document.baseURI=globalThis.__njs_base||'';"
    "document.URL=document.baseURI;document.location=globalThis.location;}"
    "globalThis.matchMedia=function(){return{matches:false,media:'',addListener:sink,"
    "removeListener:sink,addEventListener:sink,removeEventListener:sink};};"
    "globalThis.getComputedStyle=function(){return{getPropertyValue:function(){return'';}};};"
    "globalThis.Node=globalThis.Node||function(){};"
    "globalThis.Element=globalThis.Element||function(){};"
    "globalThis.HTMLElement=globalThis.HTMLElement||function(){};"
    "globalThis.HTMLIFrameElement=globalThis.HTMLIFrameElement||function(){};"
    "globalThis.SVGElement=globalThis.SVGElement||function(){};"
    "globalThis.Document=globalThis.Document||function(){};"
    "globalThis.Text=globalThis.Text||function(){};"
    "globalThis.Comment=globalThis.Comment||function(){};"
    /* An event a script builds has to carry what it was built with. These
     * were empty functions, so `new CustomEvent('x',{detail:d})` produced an
     * object with no type and no detail, and the handler that received it
     * could not tell what had happened. */
    "globalThis.Event=function(t,o){o=o||{};this.type=t;this.bubbles=!!o.bubbles;"
    "this.cancelable=!!o.cancelable;this.defaultPrevented=false;this.target=null;"
    "this.currentTarget=null;this.timeStamp=0;this.isTrusted=false;"
    "this.preventDefault=function(){if(this.cancelable)this.defaultPrevented=true;};"
    "this.stopPropagation=function(){this.__stop=true;};"
    "this.stopImmediatePropagation=function(){this.__stop=true;this.__stopnow=true;};};"
    "globalThis.CustomEvent=function(t,o){o=o||{};Event.call(this,t,o);this.detail="
    "o.detail===undefined?null:o.detail;};"
    "globalThis.MouseEvent=globalThis.KeyboardEvent=globalThis.PointerEvent="
    "globalThis.InputEvent=globalThis.FocusEvent=Event;"
    /* classList and dataset, built from the element's own attribute methods
     * so neither needs the host. classList is on nearly every page that has
     * a script at all, and dataset is how markup hands a value to one. */
    "globalThis.__njs_classlist=function(el){"
    "var g=function(){return String(el.className||'').split(/\\s+/).filter(Boolean);};"
    "var s=function(a){el.className=a.join(' ');};"
    "var o={contains:function(c){return g().indexOf(c)>=0;},"
    "add:function(){var a=g();for(var i=0;i<arguments.length;i++)"
    "if(a.indexOf(arguments[i])<0)a.push(arguments[i]);s(a);},"
    "remove:function(){var a=g();for(var i=0;i<arguments.length;i++){"
    "var k=a.indexOf(arguments[i]);if(k>=0)a.splice(k,1);}s(a);},"
    "toggle:function(c,f){var a=g(),k=a.indexOf(c);"
    "var on=f===undefined?k<0:!!f;if(on&&k<0)a.push(c);else if(!on&&k>=0)a.splice(k,1);"
    "s(a);return on;},"
    "replace:function(x,y){var a=g(),k=a.indexOf(x);if(k<0)return false;a[k]=y;s(a);return true;},"
    "item:function(i){return g()[i]||null;},"
    "toString:function(){return g().join(' ');}};"
    "Object.defineProperty(o,'length',{get:function(){return g().length;}});"
    "Object.defineProperty(o,'value',{get:function(){return g().join(' ');}});"
    "return o;};"
    /* dataset reads and writes through the attributes, so a value a script
     * sets is the one markup shows, and the kebab spelling is derived rather
     * than stored twice. */
    "globalThis.__njs_dataset=function(el){"
    "var kebab=function(p){return 'data-'+p.replace(/[A-Z]/g,function(c){"
    "return '-'+c.toLowerCase();});};"
    "var camel=function(a){return a.slice(5).replace(/-([a-z])/g,function(m,c){"
    "return c.toUpperCase();});};"
    "return new Proxy({},{get:function(t,p){if(typeof p!=='string')return undefined;"
    "var v=el.getAttribute(kebab(p));return v===null?undefined:v;},"
    "set:function(t,p,v){el.setAttribute(kebab(p),String(v));return true;},"
    "has:function(t,p){return el.hasAttribute(kebab(p));},"
    "deleteProperty:function(t,p){el.removeAttribute(kebab(p));return true;},"
    "ownKeys:function(){return el.getAttributeNames()"
    ".filter(function(a){return a.indexOf('data-')===0;}).map(camel);},"
    "getOwnPropertyDescriptor:function(t,p){return{enumerable:true,configurable:true,"
    "value:el.getAttribute(kebab(p))};}});};"
    /* Storage a page can actually use. Backed by a plain object, so it lasts
     * as long as the page rather than across a boot: a script that stores a
     * preference and reads it back in the same visit works, and nothing is
     * written to disk that the reader did not ask to keep. */
    "var mkstore=function(){var m={};return{getItem:function(k){"
    "return Object.prototype.hasOwnProperty.call(m,String(k))?m[String(k)]:null;},"
    "setItem:function(k,v){m[String(k)]=String(v);},"
    "removeItem:function(k){delete m[String(k)];},"
    "clear:function(){m={};},"
    "key:function(i){return Object.keys(m)[i]||null;},"
    "get length(){return Object.keys(m).length;}};};"
    "globalThis.localStorage=mkstore();globalThis.sessionStorage=mkstore();"
    "globalThis.MutationObserver=globalThis.MutationObserver||function(){"
    "this.observe=sink;this.disconnect=sink;this.takeRecords=function(){return[];};};"
    "if(globalThis.document){document.defaultView=globalThis;document.nodeType=9;}"
    "})();";

void njs_install_dom(JSContext *ctx, void *host) {
    JS_SetContextOpaque(ctx, host);
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue doc = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, doc, "createElement", JS_NewCFunction(ctx, doc_create_element, "createElement", 1));
    JS_SetPropertyStr(ctx, doc, "createDocumentFragment",
                      JS_NewCFunction(ctx, doc_create_fragment, "createDocumentFragment", 0));
    JS_SetPropertyStr(ctx, doc, "createTextNode", JS_NewCFunction(ctx, doc_create_text, "createTextNode", 1));
    JS_SetPropertyStr(ctx, doc, "getElementById", JS_NewCFunction(ctx, doc_get_by_id, "getElementById", 1));
    JS_SetPropertyStr(ctx, doc, "querySelector", JS_NewCFunction(ctx, doc_query, "querySelector", 1));
    JS_SetPropertyStr(ctx, doc, "querySelectorAll", JS_NewCFunction(ctx, doc_query_all, "querySelectorAll", 1));
    JS_SetPropertyStr(ctx, doc, "addEventListener", JS_NewCFunction(ctx, el_add_listener, "addEventListener", 2));
    JS_SetPropertyStr(ctx, doc, "removeEventListener",
                      JS_NewCFunction(ctx, el_remove_listener, "removeEventListener", 2));
    JS_SetPropertyStr(ctx, doc, "getElementsByTagName",
                      JS_NewCFunction(ctx, doc_by_tag, "getElementsByTagName", 1));
    JS_SetPropertyStr(ctx, doc, "getElementsByClassName",
                      JS_NewCFunction(ctx, doc_by_class, "getElementsByClassName", 1));
    JS_SetPropertyStr(ctx, doc, "body", make_element(ctx, njs_dom_body(host)));
    JS_SetPropertyStr(ctx, doc, "documentElement", make_element(ctx, njs_dom_query(host, "html")));
    JS_SetPropertyStr(ctx, doc, "head", make_element(ctx, njs_dom_query(host, "head")));
    JS_SetPropertyStr(ctx, global, "document", doc);
    /* The page's own address goes on before the prelude runs, because the
     * prelude builds location out of it. */
    install_page_address(ctx, global, host);
    JS_FreeValue(ctx, global);
    JSValue r = JS_Eval(ctx, PRELUDE, __builtin_strlen(PRELUDE), "<prelude>", JS_EVAL_TYPE_GLOBAL);
    JS_FreeValue(ctx, r);
}
