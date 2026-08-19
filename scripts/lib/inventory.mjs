import { createHash } from 'node:crypto';
import { readdir, readFile } from 'node:fs/promises';
import path from 'node:path';

export function parseUniqueTypes(source) {
  const enumStart = source.indexOf('enum class UniqueType');
  if (enumStart < 0) return [];

  const entries = [];
  const lines = source.slice(enumStart).split('\n');
  for (const line of lines) {
    if (line.trim() === ';') break;
    const match = /^ {4}([A-Za-z][A-Za-z0-9_]*)\s*\(/.exec(line);
    if (match === null) continue;
    const text = /\(\s*"([^"]*)"/.exec(line)?.[1] ?? null;
    entries.push({ name: match[1], text });
  }
  return entries;
}

export function parseEndpoints(source) {
  const endpointPattern =
    /request\s*\(\s*HttpMethod\.(Get|Post|Put|Patch|Delete)\s*,\s*"([^"]+)"([^,]*)/g;
  const endpoints = [];
  for (const match of source.matchAll(endpointPattern)) {
    const suffix = match[3];
    const hasConcatenation = /\+/.test(suffix);
    let endpointPath = match[2].replace(/\$\{?[^/}" ]+}?/g, '{dynamic}');
    if (hasConcatenation) endpointPath = `${endpointPath}{dynamic}`;
    endpoints.push({ method: match[1].toUpperCase(), path: endpointPath });
  }
  return uniqueSorted(endpoints, ({ method, path: endpointPath }) => `${method} ${endpointPath}`);
}

export function extractKotlinFields(source, file) {
  const fields = [];
  const seen = new Set();
  let inClass = false;
  let classDepth = 0;

  for (const line of source.split('\n')) {
    if (/^(?:data\s+)?(?:class|object)\s+[A-Za-z]/.test(line.trim())) {
      inClass = true;
      classDepth = 0;
    }

    if (inClass && !/\bprivate\b/.test(line)) {
      const match = /^ {4}(?:override\s+)?(?:lateinit\s+)?(var|val)\s+([A-Za-z][A-Za-z0-9_]*)/.exec(
        line,
      );
      if (match !== null && classDepth <= 1) {
        const key = `${file}:${match[2]}`;
        if (!seen.has(key)) {
          seen.add(key);
          fields.push({ file, name: match[2], mutable: match[1] === 'var' });
        }
      }
    }

    classDepth += count(line, '{') - count(line, '}');
    if (inClass && classDepth < 0) {
      inClass = false;
      classDepth = 0;
    }
  }

  return fields;
}

export function extractKotlinSymbols(source) {
  const symbols = [];
  for (const line of source.split('\n')) {
    if (/\bprivate\b/.test(line)) continue;
    const classMatch = /^\s*(?:data\s+|sealed\s+|abstract\s+|open\s+)?class\s+([A-Za-z][A-Za-z0-9_]*)/.exec(
      line,
    );
    if (classMatch !== null) symbols.push({ kind: 'class', name: classMatch[1] });
    const functionMatch = /^\s*(?:suspend\s+)?fun\s+([A-Za-z][A-Za-z0-9_]*)/.exec(line);
    if (functionMatch !== null) symbols.push({ kind: 'function', name: functionMatch[1] });
  }
  return uniqueSorted(symbols, ({ kind, name }) => `${kind}:${name}`);
}

export async function walkFiles(root, predicate = () => true) {
  const result = [];
  async function visit(directory) {
    const entries = await readdir(directory, { withFileTypes: true });
    entries.sort((left, right) => left.name.localeCompare(right.name));
    for (const entry of entries) {
      const fullPath = path.join(directory, entry.name);
      if (entry.isDirectory()) await visit(fullPath);
      else if (entry.isFile() && predicate(fullPath)) result.push(fullPath);
    }
  }
  await visit(root);
  return result;
}

export async function sha256File(file) {
  const content = await readFile(file);
  return createHash('sha256').update(content).digest('hex');
}

export function uniqueSorted(items, keyOf) {
  const byKey = new Map();
  for (const item of items) byKey.set(keyOf(item), item);
  return [...byKey.entries()]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([, item]) => item);
}

function count(value, needle) {
  return value.split(needle).length - 1;
}
