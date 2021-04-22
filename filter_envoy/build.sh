# Get nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
# Compile the filter
cargo +nightly build -Z unstable-options --target=wasm32-unknown-unknown --release --out-dir wasm_bins --target-dir target
