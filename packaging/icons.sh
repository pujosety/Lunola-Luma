#!/usr/bin/env bash
# Regenerate the approved Lunola Luma wordmarks and all platform icons.
#
# Requires Python 3 and Pillow (also used by the macOS DMG artwork generator).
# Stable app IDs and asset filenames remain `photocraft` for compatibility.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PYTHON="${PYTHON:-python3}"
command -v "$PYTHON" >/dev/null || { echo "error: Python 3 not found" >&2; exit 1; }
"$PYTHON" "$ROOT/brand/lunola-luma/generate_assets.py"
echo "Lunola Luma icons written to $ROOT/assets/app-icon"
