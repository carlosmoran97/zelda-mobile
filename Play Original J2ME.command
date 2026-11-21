#!/bin/bash
set -euo pipefail

GAME_DIR="$(cd -- "$(dirname -- "$0")" && pwd)"

JAVA_BIN=""
for candidate in \
    "${JAVA_HOME:-}/bin/java" \
    /opt/homebrew/opt/openjdk@17/bin/java \
    /opt/homebrew/opt/openjdk@21/bin/java \
    /usr/local/opt/openjdk@17/bin/java; do
    if [[ -x "$candidate" ]]; then
        JAVA_BIN="$candidate"
        break
    fi
done

if [[ -z "$JAVA_BIN" ]]; then
    JAVA_BIN="$(command -v java || true)"
fi

if [[ -z "$JAVA_BIN" ]]; then
    echo "Java is required. Install it with: brew install openjdk@17"
    read -r -p "Press Return to close."
    exit 1
fi

# Keep the emulator's settings and save data together in this folder.
cd "$GAME_DIR/emulator"
exec "$JAVA_BIN" -Dfile.encoding=ISO_8859_1 -Xdock:name="Zelda Mobile" \
    -jar freej2me_plus.jar \
    "$GAME_DIR/TheLegendOfZeldaMobile-beta4.jar" \
    0 240 320 2
