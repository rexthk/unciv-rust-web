#!/usr/bin/env bash
set -euo pipefail

oracle_dir="$(cd "$(dirname "$0")" && pwd)"
repo_dir="$(cd "$oracle_dir/.." && pwd)"
upstream_dir="${UNCIV_UPSTREAM_DIR:-/Users/rexthk/repo/Unciv}"
export JAVA_HOME="${JAVA_HOME:-/opt/homebrew/opt/openjdk@21}"

"$upstream_dir/gradlew" \
  -p "$oracle_dir" \
  --quiet \
  run \
  --args="$(cd "$repo_dir" && realpath "$1")"
