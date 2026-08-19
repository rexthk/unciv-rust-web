import assert from 'node:assert/strict';
import test from 'node:test';

import {
  extractKotlinFields,
  extractKotlinSymbols,
  parseEndpoints,
  parseUniqueTypes,
} from './inventory.mjs';

test('parseUniqueTypes extracts enum constants and ignores methods', () => {
  const source = `
enum class UniqueType(val text: String) {
    First("first"),
    @Deprecated("old")
    Second(
        "second",
    );

    fun helper() = Unit
}
`;

  assert.deepEqual(
    parseUniqueTypes(source).map(({ name }) => name),
    ['First', 'Second'],
  );
});

test('parseEndpoints extracts HTTP method and literal endpoint', () => {
  const source = `
request(HttpMethod.Post, "api/v2/auth/login", client, helper)
request(
    HttpMethod.Get,
    "api/v2/game/" + gameUUID,
    client,
    helper,
)
`;

  assert.deepEqual(parseEndpoints(source), [
    { method: 'GET', path: 'api/v2/game/{dynamic}' },
    { method: 'POST', path: 'api/v2/auth/login' },
  ]);
});

test('extractKotlinFields records class-level mutable and immutable properties', () => {
  const source = `
class GameInfo(
    var turns: Int = 0,
) {
    val gameId: String = ""
    private val cache = mutableMapOf<String, String>()

    fun local() {
        val ignored = 1
    }
}
`;

  assert.deepEqual(extractKotlinFields(source, 'GameInfo.kt'), [
    { file: 'GameInfo.kt', name: 'turns', mutable: true },
    { file: 'GameInfo.kt', name: 'gameId', mutable: false },
  ]);
});

test('extractKotlinSymbols returns declared classes and functions', () => {
  const source = `
class EditorTool {
    fun paintTile() = Unit
    private fun helper() = Unit
}
`;

  assert.deepEqual(extractKotlinSymbols(source), [
    { kind: 'class', name: 'EditorTool' },
    { kind: 'function', name: 'paintTile' },
  ]);
});
