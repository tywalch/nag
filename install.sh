#!/bin/bash

set -e

BINARY_TARGET="$(pwd)/target/release/nag"
# INSTALL_TARGET is either the first param or the default
INSTALL_TARGET_DIR="${1:-/usr/local/bin}"
INSTALL_TARGET="${INSTALL_TARGET_DIR}/nag"

cargo build --release

echo "#!/bin/bash
if [ "\$#" -lt 2 ]; then
    echo \"Usage: nag <duration> <message1> [message2] ...\"
    exit 1
fi

DURATION=\$1

shift

ARGS="\$*"

$BINARY_TARGET -e \$DURATION \"\$ARGS\"

nohup $BINARY_TARGET \$DURATION \"\$ARGS\" > /dev/null 2>&1 &
" > "$INSTALL_TARGET"

chmod +x "$INSTALL_TARGET"