# Install rust toolchain and cross compile the app for the raspberry pi

```shell
brew uninstall rust
brew install rustup

rustup default stable
rustup target add armv7-unknown-linux-gnueabihf
cargo install cross --git https://github.com/cross-rs/cross
cross build --target armv7-unknown-linux-gnueabihf --release

rsync -avz target/armv7-unknown-linux-gnueabihf/release/vpn-toggle root@rpi-vpn:/usr/local/bin/
```

# Install as systemd service

See [here](deploy/README.md).
