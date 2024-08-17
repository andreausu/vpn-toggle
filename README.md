# Installazione rust toolchain e cross compile per il rpi

```
brew uninstall rust
brew install rustup

rustup default stable
rustup target add armv7-unknown-linux-gnueabihf # per il rpi
cross build --target armv7-unknown-linux-gnueabihf --release

rsync -avz target/armv7-unknown-linux-gnueabihf/release/vpn-toggle root@192.168.1.48:/usr/local/bin/
```

# Installazione come service systemd

See [here](https://gitlab.com/Usu/home-provisioning/-/tree/master/rpi-vpn?ref_type=heads).
