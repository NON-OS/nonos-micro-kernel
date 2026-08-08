/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * The rest of the element surface. Included by dom_bindings.c after the
 * statics it builds on, and kept apart from them because it is a list of
 * ordinary APIs rather than machinery: everything here is one call through
 * to the host tree.
 */

extern int njs_dom_clone_node(void *host, int node, int deep);
extern int njs_dom_create_fragment(void *host);
extern int njs_dom_last_child(void *host, int node);
extern int njs_dom_prev_sibling(void *host, int node);
extern int njs_dom_replace_child(void *host, int parent, int fresh, int old);
extern int njs_dom_has_attr(void *host, int node, const char *k);
extern int njs_dom_contains(void *host, int node, int other);

/* A page writes a row's shape once and every row is a copy of it. Without
 * this a script has to build each row tag by tag, which is not what the code
 * on the page does, so the list renders empty. */
static JSValue el_clone_node(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    int deep = argc > 0 && JS_ToBool(ctx, v[0]) > 0;
    return make_element(ctx, njs_dom_clone_node(JS_GetContextOpaque(ctx), node_of(ctx, t), deep));
}

/* The new node has to land where the old one sat. Appending and removing
 * would put it at the end, so a replaced row jumps to the bottom. */
static JSValue el_replace_child(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 2) return JS_UNDEFINED;
    void *host = JS_GetContextOpaque(ctx);
    int old = njs_dom_replace_child(host, node_of(ctx, t), node_of(ctx, v[0]), node_of(ctx, v[1]));
    return old < 0 ? JS_UNDEFINED : JS_DupValue(ctx, v[1]);
}

static JSValue el_contains(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1 || !JS_IsObject(v[0])) return JS_FALSE;
    void *host = JS_GetContextOpaque(ctx);
    return njs_dom_contains(host, node_of(ctx, t), node_of(ctx, v[0])) ? JS_TRUE : JS_FALSE;
}

/* An attribute that is present with an empty value is how markup writes the
 * ones that are simply on, so reading the value cannot answer this. */
static JSValue el_has_attribute(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_FALSE;
    const char *k = JS_ToCString(ctx, v[0]);
    if (!k) return JS_FALSE;
    int has = njs_dom_has_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), k);
    JS_FreeCString(ctx, k);
    return has ? JS_TRUE : JS_FALSE;
}

static JSValue doc_create_fragment(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)t; (void)a; (void)v;
    return make_element(ctx, njs_dom_create_fragment(JS_GetContextOpaque(ctx)));
}

static JSValue get_id(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    char *s = njs_dom_get_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), "id");
    if (!s) return JS_NewString(ctx, "");
    JSValue r = JS_NewString(ctx, s);
    free(s);
    return r;
}
static JSValue get_last_child(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return make_element(ctx, njs_dom_last_child(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue get_prev_sibling(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return make_element(ctx, njs_dom_prev_sibling(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
/* parentElement stops at the document, where parentNode does not. A walk up
 * that does not stop runs off the top of the tree. */
static JSValue get_parent_element(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    void *host = JS_GetContextOpaque(ctx);
    int p = njs_dom_parent(host, node_of(ctx, t));
    if (p < 0 || njs_dom_node_kind(host, p) != 1) return JS_NULL;
    return make_element(ctx, p);
}
/* children skips text, where childNodes does not. */
static JSValue get_children(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    void *host = JS_GetContextOpaque(ctx);
    int n = node_of(ctx, t), count = njs_dom_child_count(host, n);
    JSValue arr = JS_NewArray(ctx);
    uint32_t at = 0;
    for (int i = 0; i < count; i++) {
        int c = njs_dom_child_at(host, n, i);
        if (njs_dom_node_kind(host, c) == 1)
            JS_SetPropertyUint32(ctx, arr, at++, make_element(ctx, c));
    }
    return arr;
}

/* Properties that are stored as attributes. `el.href = "/a"` and
 * setAttribute("href", "/a") are the same write, and answering undefined for
 * the property form leaves a link a script built with no destination. */
static const char *const g_reflected[][2] = {
    {"href", "href"}, {"src", "src"}, {"alt", "alt"}, {"title", "title"},
    {"type", "type"}, {"name", "name"}, {"rel", "rel"}, {"target", "target"},
    {"placeholder", "placeholder"}, {"action", "action"}, {"method", "method"},
    {"role", "role"}, {"lang", "lang"}, {"dir", "dir"}, {"content", "content"},
    {"srcset", "srcset"}, {"sizes", "sizes"}, {"loading", "loading"},
    {"htmlFor", "for"}, {"tabIndex", "tabindex"}, {"colSpan", "colspan"},
    {"rowSpan", "rowspan"}, {"maxLength", "maxlength"}, {"value", "value"}, {0, 0},
};

/* Properties that are present or absent rather than set to a string. Writing
 * "false" would leave a control disabled by the code that meant to enable
 * it, so false removes the attribute instead. */
static const char *const g_presence[] = {
    "disabled", "checked", "readOnly", "required", "multiple", "selected", "open", "hidden", 0,
};
static const char *const g_presence_attr[] = {
    "disabled", "checked", "readonly", "required", "multiple", "selected", "open", "hidden", 0,
};

/* The reflected accessors share one pair of functions and take the table row
 * as their magic, so adding a property is a line in the table above rather
 * than another pair of functions. */
static JSValue get_reflected(JSContext *ctx, JSValueConst t, int a, JSValueConst *v, int magic) {
    (void)a; (void)v;
    char *s = njs_dom_get_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), g_reflected[magic][1]);
    if (!s) return JS_NewString(ctx, "");
    JSValue r = JS_NewString(ctx, s);
    free(s);
    return r;
}
static JSValue set_reflected(JSContext *ctx, JSValueConst t, int a, JSValueConst *v, int magic) {
    if (a < 1) return JS_UNDEFINED;
    const char *s = JS_ToCString(ctx, v[0]);
    if (s) {
        njs_dom_set_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), g_reflected[magic][1], s);
        JS_FreeCString(ctx, s);
    }
    return JS_UNDEFINED;
}
static JSValue get_presence(JSContext *ctx, JSValueConst t, int a, JSValueConst *v, int magic) {
    (void)a; (void)v;
    void *host = JS_GetContextOpaque(ctx);
    return njs_dom_has_attr(host, node_of(ctx, t), g_presence_attr[magic]) ? JS_TRUE : JS_FALSE;
}
static JSValue set_presence(JSContext *ctx, JSValueConst t, int a, JSValueConst *v, int magic) {
    if (a < 1) return JS_UNDEFINED;
    void *host = JS_GetContextOpaque(ctx);
    int n = node_of(ctx, t);
    if (JS_ToBool(ctx, v[0]) > 0)
        njs_dom_set_attr(host, n, g_presence_attr[magic], "");
    else
        njs_dom_remove_attr(host, n, g_presence_attr[magic]);
    return JS_UNDEFINED;
}

static void accessor_magic(JSContext *ctx, JSValueConst el, const char *name,
                           JSCFunctionMagic *get, JSCFunctionMagic *set, int magic) {
    JSAtom a = JS_NewAtom(ctx, name);
    JSValue g = JS_NewCFunctionMagic(ctx, get, name, 0, JS_CFUNC_getter_magic, magic);
    JSValue s = JS_NewCFunctionMagic(ctx, set, name, 1, JS_CFUNC_setter_magic, magic);
    JS_DefinePropertyGetSet(ctx, el, a, g, s, JS_PROP_C_W_E);
    JS_FreeAtom(ctx, a);
}

/* Everything above, hung on one element. Called from make_element so a node
 * gets the whole surface the moment a script first reaches it. */
static void install_element_ext(JSContext *ctx, JSValueConst el) {
    JS_SetPropertyStr(ctx, el, "cloneNode", JS_NewCFunction(ctx, el_clone_node, "cloneNode", 1));
    JS_SetPropertyStr(ctx, el, "replaceChild",
                      JS_NewCFunction(ctx, el_replace_child, "replaceChild", 2));
    JS_SetPropertyStr(ctx, el, "contains", JS_NewCFunction(ctx, el_contains, "contains", 1));
    JS_SetPropertyStr(ctx, el, "hasAttribute",
                      JS_NewCFunction(ctx, el_has_attribute, "hasAttribute", 1));
    accessor(ctx, el, "id", get_id, set_id);
    accessor(ctx, el, "lastChild", get_last_child, 0);
    accessor(ctx, el, "previousSibling", get_prev_sibling, 0);
    accessor(ctx, el, "parentElement", get_parent_element, 0);
    accessor(ctx, el, "children", get_children, 0);
    for (int i = 0; g_reflected[i][0]; i++)
        accessor_magic(ctx, el, g_reflected[i][0], get_reflected, set_reflected, i);
    for (int i = 0; g_presence[i]; i++)
        accessor_magic(ctx, el, g_presence[i], get_presence, set_presence, i);
}
