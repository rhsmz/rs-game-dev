#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="$(tr -d ' \r\n' <"$ROOT/third_party/filament_prebuilt/VERSION")"

os="$(uname -s)"
case "$os" in
  Linux)
    pkg="filament-${VERSION}-linux.tgz"
    dest="$ROOT/third_party/filament_prebuilt/linux-x86_64"
    ;;
  Darwin)
    pkg="filament-${VERSION}-mac.tgz"
    dest="$ROOT/third_party/filament_prebuilt/macos-arm64"
    ;;
  *)
    echo "Unsupported OS: $os" >&2
    exit 1
    ;;
esac

url="https://github.com/google/filament/releases/download/${VERSION}/${pkg}"
tmp="${TMPDIR:-/tmp}/${pkg}"

echo "Downloading $url"
curl -sL -o "$tmp" "$url"

find "$dest" -mindepth 1 -maxdepth 1 ! -name '.gitkeep' -exec rm -rf {} +

mkdir -p "$dest"
tar -xf "$tmp" -C "$dest"

case "$os" in
  Linux)
    lib="$dest/filament/lib/x86_64"
    ;;
  Darwin)
    lib="$dest/filament/lib/arm64"
    ;;
esac

echo "Extracted to $dest"
echo "Suggested FILAMENT_LIB_DIR=$lib"
