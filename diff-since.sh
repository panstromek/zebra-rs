#!/usr/bin/env bash
git diff --stat "$1" HEAD | tail -n 1
echo diff: $(($(git diff --stat "$1" HEAD | tail -n 1 | rg "([0-9]+) insertions.*? ([0-9]+) deletions" --replace "\$1 - \$2" -o)))
