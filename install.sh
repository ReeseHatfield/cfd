#!/bin/bash

# users gotta have cargo 
if ! command -v cargo &> /dev/null; then
    echo "Cargo is a prereq. and must be installed to isntall cfd"
    exit 1
fi

# steal from run.sh
RUSTFLAGS="-D warnings" cargo build --release --target-dir bin

# did the build actually build or nah?
if [ $? -ne 0 ]; then
    echo "Build failed."
    exit 1
fi

INSTALL_DIR="/usr/local/bin"
EXECUTABLE_NAME="cfd"  

cp bin/release/$EXECUTABLE_NAME "$INSTALL_DIR"

echo "Installed $EXECUTABLE_NAME to $INSTALL_DIR."
