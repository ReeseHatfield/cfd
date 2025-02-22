#!/bin/bash

# users gotta have cargo 

# export PATH="$HOME/.local/bin:$PATH"

# steal from run.sh
RUSTFLAGS="-D warnings" cargo build --release --target-dir bin

# did the build actually build or nah?
if [ $? -ne 0 ]; then
    echo "Build failed."
    exit 1
fi

INSTALL_DIR="$HOME/.local/bin"
EXECUTABLE_NAME="cfd"  

cp bin/release/$EXECUTABLE_NAME "$INSTALL_DIR"

CONFIG_DIR="${HOME}/.config/cfd"

mkdir -p "${CONFIG_DIR}"

echo "creating config"
cat > "${CONFIG_DIR}/cfd.json"<< EOF
{
  "editor": "Neovim"
}
EOF

echo "Installed $EXECUTABLE_NAME to $INSTALL_DIR."
