// NONOS Operating System (AGPL-3.0-or-later)
// Pull the prelude out of the C source it ships in, so these checks run the
// text the engine actually evaluates rather than a copy that drifts from it.

import { readFileSync } from 'node:fs';

const START = 'static const char *PRELUDE =';

export function extractPrelude(cPath) {
  const lines = readFileSync(cPath, 'utf8').split('\n');
  const at = lines.findIndex(l => l.startsWith(START));
  if (at < 0) throw new Error(`no prelude in ${cPath}`);

  let out = '';
  for (const line of lines.slice(at + 1)) {
    const text = line.trim();
    // The definition ends at the statement's semicolon on its own.
    if (text === ';') return out;
    // Comments sit between the string pieces and are not part of the value.
    if (text.startsWith('/*') || text.startsWith('*') || text === '') continue;
    const piece = text.match(/^"(.*)"\s*;?$/);
    if (!piece) throw new Error(`unparsed prelude line: ${text}`);
    out += unescapeC(piece[1]);
    if (text.endsWith(';')) return out;
  }
  throw new Error('prelude never terminated');
}

// Only the escapes the prelude actually uses. Anything else is a mistake
// worth failing on rather than passing through and behaving differently in
// the engine than it does here.
function unescapeC(s) {
  let out = '';
  for (let i = 0; i < s.length; i++) {
    if (s[i] !== '\\') {
      out += s[i];
      continue;
    }
    const next = s[++i];
    if (next === '\\') out += '\\';
    else if (next === '"') out += '"';
    else if (next === 'n') out += '\n';
    else throw new Error(`unhandled escape \\${next}`);
  }
  return out;
}
