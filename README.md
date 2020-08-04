# candev

![mit](https://img.shields.io/github/license/reneherrero/candev)
![build](https://img.shields.io/github/workflow/status/reneherrero/candev/Continuous%20Integration)

SocketCAN based *experimental* library that implements @timokroeger's proposed ([PR](https://github.com/rust-embedded/embedded-hal/pull/212)) [embedded-hal](https://github.com/timokroeger/embedded-hal/tree/can) CAN traits.

## Running the examples

### Prerequisites

1. Make sure the `can-utils` package in installed on your system
2. Setup Virtual CAN device as follows:
```bash
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
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

## Credits

This project initial starting point was based off [socketcan-rs](https://github.com/mbr/socketcan-rs).

Big thanks to @timokroeger for his help and contributions to the [embedded-hal](https://github.com/rust-embedded/embedded-hal) project.