#!/bin/bash
set -euo pipefail

GAME_DIR="$(cd -- "$(dirname -- "$0")" && pwd)"

APP_PATH="$GAME_DIR/dist/Zelda Mobile.app"
if [[ ! -x "$APP_PATH/Contents/MacOS/zelda-mobile" && -x "/Applications/Zelda Mobile.app/Contents/MacOS/zelda-mobile" ]]; then
    APP_PATH="/Applications/Zelda Mobile.app"
fi

if [[ ! -x "$APP_PATH/Contents/MacOS/zelda-mobile" ]]; then
    echo "The native app has not been built yet."
    echo "From this folder, run:"
    echo "  python3 scripts/bootstrap.py"
    echo "  python3 scripts/package.py"
    read -r -p "Press Return to close."
    exit 1
fi

exec /usr/bin/open "$APP_PATH"
