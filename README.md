## The [socketcan](https://github.com/socketcan-rs/socketcan-rs) crate has support for the embedded-can crates since version 2.0.0

# candev

![ci](https://github.com/reneherrero/candev/workflows/ci/badge.svg)
![license](https://img.shields.io/badge/license-MIT%20or%20Apache--2-brightgreen)

SocketCAN based experimental library that implements proposed ([PR](https://github.com/rust-embedded/embedded-hal/pull/212)) `embedded-hal` CAN traits.

## Running the examples

### Prerequisites

1. Make sure the `can-utils` package in installed on your system
2. Setup Virtual CAN device as follows:

```bash
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

Note that this can be automated at startup by:

1. Adding a line with `vcan` in `/etc/modules`
2. Adding the following to `/etc/network/interfaces`

```bash
auto vcan0
iface vcan0 inet manual
pre-up /sbin/ip link add dev $IFACE type vcan
up /sbin/ip link set up $IFACE
down /sbin/ip link set down $IFACE
```

### Receive

A simple example on how to receive a CAN frame

1. Run `cargo run --example receive` in a terminal
2. Run `cansend vcan0 123#DEADBEEF` in another terminal

You should see `123#DEADBEEF` (or whatever id and data you passed to cansend) appear on the first terminal.

### Transmit

A simple example on how to transmit a CAN frame.

1. Run `candump vcan0` in a terminal
2. Run `cargo run --example transmit` in another terminal

You should see `candump` output `vcan0  001   [4]  DE AD BE FF`.

### Driver

A sample driver `embedded-hal` CAN driver implementation that makes use of candev and simple echoes a message. In this example, two instances of the driver ping pong a message to each other.

To observe the behavior, do as follows:

1. Run `candump vcan0` in a terminal
2. Run `cargo run --example driver` in another terminal

You'll be able to observe the interaction of the two driver instances on the first terminal.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.64 and up. It *might* compile with older versions but that may change in any new patch release.

## License

Licensed under either of

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

This project initial starting point was based off [socketcan-rs](https://github.com/mbr/socketcan-rs).

Big thanks to [Timo](https://github.com/timokroeger) for his help and developing the CAN traits for the [embedded-hal](https://github.com/rust-embedded/embedded-hal) project.
