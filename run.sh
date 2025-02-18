mkdir -p bin

cargo build --release --target-dir bin

./bin/release/cfd "$@"

