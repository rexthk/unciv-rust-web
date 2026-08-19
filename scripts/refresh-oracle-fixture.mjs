import { execFileSync } from 'node:child_process';
import { writeFile } from 'node:fs/promises';
import path from 'node:path';

const root = path.resolve('.');
const scenario = 'fixtures/unciv-4.21.10/scenarios/smoke.json';
const output = execFileSync(path.join(root, 'oracle-kotlin/run-oracle.sh'), [scenario], {
  cwd: root,
  encoding: 'utf8',
  env: process.env,
  timeout: 180_000,
});
const marker = '@@UNCIV_ORACLE_JSON@@';
const payload = output
  .split('\n')
  .find((line) => line.startsWith(marker))
  ?.slice(marker.length);
if (payload === undefined) throw new Error('Oracle marker not found');
const normalized = JSON.parse(payload);
await writeFile(
  path.join(root, 'fixtures/unciv-4.21.10/normalized/smoke.json'),
  `${JSON.stringify(normalized, null, 2)}\n`,
);
console.log('Refreshed normalized smoke fixture.');
