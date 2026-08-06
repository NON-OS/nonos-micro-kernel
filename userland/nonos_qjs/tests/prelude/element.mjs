// NONOS Operating System (AGPL-3.0-or-later)
// A stand-in element carrying the four attribute methods the prelude
// factories build on. The real ones cross into the host tree; what is under
// test here is the JS on top of them.

export function mkEl() {
  const attrs = {};
  return {
    className: '',
    getAttribute: k => (k in attrs ? attrs[k] : null),
    setAttribute: (k, v) => { attrs[k] = String(v); },
    removeAttribute: k => { delete attrs[k]; },
    hasAttribute: k => k in attrs,
    getAttributeNames: () => Object.keys(attrs),
  };
}
