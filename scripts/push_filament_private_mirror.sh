#!/usr/bin/env bash
set -euo pipefail
PRIVATE_URL="git@github.com:rhsmz/filament-rs-game-dev.git"
REPO_SLUG="rhsmz/filament-rs-game-dev"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/third_party/filament"

echo "Pushing $(git rev-parse --short HEAD) to $PRIVATE_URL (branch private) ..."
git push "$PRIVATE_URL" HEAD:refs/heads/private

if git rev-parse -q --verify refs/tags/v1.70.1 >/dev/null 2>&1; then
  echo "Pushing tag v1.70.1 ..."
  git push "$PRIVATE_URL" refs/tags/v1.70.1
fi

echo "Setting default branch to private ..."
gh repo edit "$REPO_SLUG" --default-branch private

echo "Done."
