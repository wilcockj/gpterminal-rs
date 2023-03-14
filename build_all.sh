echo "Building windows exe"
cargo build --target x86_64-pc-windows-gnu --release
echo "Building linux executable"
cargo build --release
