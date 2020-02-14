INSTALLATION
---

Download latest version of
[leds](https://github.com/czervenka/spi-leds/releases/download/v0.1.0/leds)
file and copy it to your Raspberry.

If you led strip is not connected to default GPIO pins dedicated for SPI as
recommended, copy the [sample configuration file](./.leds) next to the binary
`leds` and modify it according to your setup (e.g. if your binary is in
`/home/pi/leds` the configuration file should live on path
`/home/pi/.leds`).



BUILD
--

*This instruction is a snipped from [Cross compiling Rust for ARM (e.g.
Raspberry Pi) using any
OS!](https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b).
(Give the author a clap.) ...*

**[Install Rust](https://www.rust-lang.org/tools/install) lang** if you do not have it installed yet.

	curl https://sh.rustup.rs | sh

**Setup arm target
	source ~/.cargo/envsudo apt-get install -qq gcc-arm-linux-gnueabihf
	rustup target add armv7-unknown-linux-gnueabihf

    mkdir -p ~/.cargo
	cat >>~/.cargo/config <<EOF
    [target.armv7-unknown-linux-gnueabihf]
    linker = "arm-linux-gnueabihf-gcc"
    EOF

**Build** the leds binary:

    cargo build --release --target=armv7-unknown-linux-gnueabihf
