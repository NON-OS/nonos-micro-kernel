// NONOS Operating System (AGPL-3.0-or-later)
// location said http://localhost/ no matter what had been fetched, so a page
// that reads its own address was told something that was never true.

export function locationChecks(ok) {
  const at = u => globalThis.__njs_mkloc(u);

  const l = at('https://example.org:8443/docs/intro?q=rust#top');
  ok(l.protocol === 'https:', `protocol: ${l.protocol}`);
  ok(l.host === 'example.org:8443', `host keeps the port: ${l.host}`);
  ok(l.hostname === 'example.org', `hostname drops it: ${l.hostname}`);
  ok(l.port === '8443', `port: ${l.port}`);
  ok(l.pathname === '/docs/intro', `pathname: ${l.pathname}`);
  ok(l.search === '?q=rust', `search: ${l.search}`);
  ok(l.hash === '#top', `hash: ${l.hash}`);
  ok(l.origin === 'https://example.org:8443', `origin: ${l.origin}`);
  ok(String(l) === l.href, 'it stringifies to its href');

  // No port is the common case, and a hostname that kept a colon would not
  // match anything a page compares it against.
  const plain = at('http://example.org/');
  ok(plain.hostname === 'example.org', `no port leaves the hostname alone: ${plain.hostname}`);
  ok(plain.port === '', 'no port reports empty');
  ok(plain.search === '' && plain.hash === '', 'absent parts are empty, not undefined');

  // A bare host with no trailing slash still has a path, because code that
  // reads pathname and indexes into it would otherwise throw.
  const bare = at('http://example.org');
  ok(bare.pathname === '/', `a missing path is the root: ${bare.pathname}`);

  // An address with a colon in the path must not be read as a port.
  const colon = at('http://example.org/a:b');
  ok(colon.hostname === 'example.org', `a colon in the path is not a port: ${colon.hostname}`);
}
