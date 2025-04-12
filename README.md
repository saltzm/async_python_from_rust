

```
cargo init --lib

cargo install --force cbindgen

cargo build --release

cbindgen --config cbindgen.toml --crate async_rust_from_python --output my_library.h

gcc main.c -L./target/debug -lasync_rust_from_python    
```                     