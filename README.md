

cargo init --lib
cargo install --force cbindgen
cargo build --release
cbindgen --config cbindgen.toml --crate async_rust_from_python --output lib.h