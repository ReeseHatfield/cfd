mkdir -p bin

# take the go approach for warnings as errors
# mostly because they will show in stdout in a build, so fix them now
RUSTFLAGS="-D warnings" cargo build --release --target-dir bin

./bin/release/cfd "$@"

