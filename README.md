cross-compile on Apple Silicon

1. brew install mingw-w64
2. cargo build --target x86_64-pc-windows-gnu --release
