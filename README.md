# Install rust toolchain and cross compile the app for the raspberry pi

```shell
brew uninstall rust
brew install rustup

rustup default stable
rustup target add armv7-unknown-linux-gnueabihf
cross build --target armv7-unknown-linux-gnueabihf --release

rsync -avz target/armv7-unknown-linux-gnueabihf/release/vpn-toggle root@192.168.1.48:/usr/local/bin/
```

# Install as systemd service

See [here](deploy/README.md).
