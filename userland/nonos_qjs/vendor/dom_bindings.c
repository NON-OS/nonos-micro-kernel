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

static JSValue ev_noop(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)ctx; (void)t; (void)argc; (void)v;
    return JS_UNDEFINED;
}

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
    JS_SetPropertyStr(ctx, ev, "preventDefault", JS_NewCFunction(ctx, ev_noop, "preventDefault", 0));
    JS_SetPropertyStr(ctx, ev, "stopPropagation", JS_NewCFunction(ctx, ev_noop, "stopPropagation", 0));
    JS_SetPropertyStr(ctx, ev, "stopImmediatePropagation", JS_NewCFunction(ctx, ev_noop, "stopImmediatePropagation", 0));
    int hops = 0;
    for (int cur = node; cur >= 0 && hops < 512; cur = njs_dom_parent(host, cur), hops++) {
        for (int i = 0; i < g_lcount; i++) {
            if (g_listeners[i].live && g_listeners[i].node == cur && type_eq(g_listeners[i].type, type)) {
                /* Listener `this` is the element it registered on; delegated
                 * handlers (Preact's eventProxy) read their table off it. */
                JSValue self = make_element(ctx, cur);
                JS_SetPropertyStr(ctx, ev, "currentTarget", JS_DupValue(ctx, self));
                JSValue arg = ev;
                JSValue r = JS_Call(ctx, g_listeners[i].fn, self, 1, (JSValueConst *)&arg);
                JS_FreeValue(ctx, r);
                JS_FreeValue(ctx, self);
                fired++;
            }
        }
        if (cur == 0) break;
    }
    /* Document-level listeners registered through document.addEventListener
     * carry node -1 and fire last, as the outermost bubble stop. */
    for (int i = 0; i < g_lcount; i++) {
        if (g_listeners[i].live && g_listeners[i].node == -1 && type_eq(g_listeners[i].type, type)) {
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
    JS_FreeValue(ctx, global);
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
    accessor(ctx, el, "id", 0, set_id);
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
    accessor(ctx, el, "innerHTML", 0, set_inner_html);
    JS_SetPropertyUint32(ctx, reg, (uint32_t)node, JS_DupValue(ctx, el));
    JS_FreeValue(ctx, reg);
    JS_FreeValue(ctx, global);
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
    "globalThis.setTimeout=function(f){q.push(f);return q.length;};"
    "globalThis.setInterval=function(){return 0;};"
    "globalThis.clearTimeout=sink;globalThis.clearInterval=sink;"
    "globalThis.requestAnimationFrame=function(f){q.push(f);return q.length;};"
    "globalThis.cancelAnimationFrame=sink;"
    "globalThis.queueMicrotask=globalThis.queueMicrotask||function(f){Promise.resolve().then(f);};"
    "globalThis.__njs_flush_timers=function(){var n=0;"
    "while(q.length&&n<10000){var f=q.shift();n++;try{f();}catch(e){"
    "(globalThis.__njs_errors=globalThis.__njs_errors||[]).push(String(e)+' @ '+String(e&&e.stack||''));}}return n;};"
    "globalThis.navigator={userAgent:'NONOS Browser',language:'en-US',languages:['en-US']};"
    "globalThis.location={href:'http://localhost/',protocol:'http:',host:'localhost',"
    "hostname:'localhost',pathname:'/',search:'',hash:'',origin:'http://localhost'};"
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
    "globalThis.Event=globalThis.Event||function(){};"
    "globalThis.CustomEvent=globalThis.CustomEvent||function(){};"
    "globalThis.MutationObserver=globalThis.MutationObserver||function(){"
    "this.observe=sink;this.disconnect=sink;this.takeRecords=function(){return[];};};"
    "if(globalThis.document){document.defaultView=globalThis;document.nodeType=9;}"
    "})();";

void njs_install_dom(JSContext *ctx, void *host) {
    JS_SetContextOpaque(ctx, host);
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue doc = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, doc, "createElement", JS_NewCFunction(ctx, doc_create_element, "createElement", 1));
    JS_SetPropertyStr(ctx, doc, "createTextNode", JS_NewCFunction(ctx, doc_create_text, "createTextNode", 1));
    JS_SetPropertyStr(ctx, doc, "getElementById", JS_NewCFunction(ctx, doc_get_by_id, "getElementById", 1));
    JS_SetPropertyStr(ctx, doc, "querySelector", JS_NewCFunction(ctx, doc_query, "querySelector", 1));
    JS_SetPropertyStr(ctx, doc, "querySelectorAll", JS_NewCFunction(ctx, doc_query_all, "querySelectorAll", 1));
    JS_SetPropertyStr(ctx, doc, "addEventListener", JS_NewCFunction(ctx, el_add_listener, "addEventListener", 2));
    JS_SetPropertyStr(ctx, doc, "body", make_element(ctx, njs_dom_body(host)));
    JS_SetPropertyStr(ctx, doc, "documentElement", make_element(ctx, njs_dom_query(host, "html")));
    JS_SetPropertyStr(ctx, doc, "head", make_element(ctx, njs_dom_query(host, "head")));
    JS_SetPropertyStr(ctx, global, "document", doc);
    JS_FreeValue(ctx, global);
    JSValue r = JS_Eval(ctx, PRELUDE, __builtin_strlen(PRELUDE), "<prelude>", JS_EVAL_TYPE_GLOBAL);
    JS_FreeValue(ctx, r);
}
