#!/usr/bin/env bash
set -euo pipefail

upstream_dir="${UNCIV_UPSTREAM_DIR:-/Users/rexthk/repo/Unciv}"
expected_commit="19de45e6eef6245f1a9774b2e1a080a0f1c87174"
expected_tree="71c09d4ecc91656f9bcf301ac0ca0e1e44604205"

if [[ ! -d "$upstream_dir/.git" ]]; then
  echo "Unciv checkout not found at $upstream_dir" >&2
  exit 1
fi

actual_commit="$(git -C "$upstream_dir" rev-parse HEAD)"
actual_tree="$(git -C "$upstream_dir" rev-parse 'HEAD^{tree}')"

[[ "$actual_commit" == "$expected_commit" ]] || {
  echo "Expected Unciv commit $expected_commit, found $actual_commit" >&2
  exit 1
}
[[ "$actual_tree" == "$expected_tree" ]] || {
  echo "Expected Unciv tree $expected_tree, found $actual_tree" >&2
  exit 1
}
grep -Fq 'const val appVersion = "4.21.10"' \
  "$upstream_dir/buildSrc/src/main/kotlin/BuildConfig.kt"

verify_sha256() {
  local expected="$1"
  local file="$2"
  local actual
  if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "$file" | cut -d' ' -f1)"
  else
    actual="$(shasum -a 256 "$file" | cut -d' ' -f1)"
  fi
  [[ "$actual" == "$expected" ]] || {
    echo "Checksum mismatch for $file" >&2
    exit 1
  }
}

verify_sha256 "36ab55bf03cccc319079974a2142f444aff842864bc32ff99c2c540544907d95" \
  "$upstream_dir/buildSrc/src/main/kotlin/BuildConfig.kt"
verify_sha256 "2ec2b371cfac8e169c157f2c3f3b37a1b8c3591a883af2d57aaf8b982d84d136" \
  "$upstream_dir/core/src/com/unciv/models/ruleset/unique/UniqueType.kt"
verify_sha256 "0f2421752ce7c5f0c2d04b311bd38113f08b5d42de9a791817318d2c72e4887a" \
  "$upstream_dir/core/src/com/unciv/logic/multiplayer/apiv2/EndpointImplementations.kt"

echo "Verified Unciv 4.21.10 at $actual_commit"
