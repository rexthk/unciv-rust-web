import { readFile } from 'node:fs/promises';

const tasks = JSON.parse(await readFile('docs/task-ledger/compatibility-tasks.json', 'utf8'));
const ids = new Set();
for (const task of tasks) {
  if (ids.has(task.id)) throw new Error(`Duplicate task ID: ${task.id}`);
  ids.add(task.id);
  if (task.status !== 'pending') throw new Error(`New task is not pending: ${task.id}`);
  if (!task.kind || !task.source) throw new Error(`Incomplete task: ${task.id}`);
}

if (tasks.length < 1_000) {
  throw new Error(`Expected at least 1000 generated parity tasks, found ${tasks.length}`);
}

console.log(`Task ledger contains ${tasks.length} unique pending tasks.`);
