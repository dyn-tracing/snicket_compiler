# Get nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
# Compile the filter
cargo +nightly build --target=wasm32-unknown-unknown --release
