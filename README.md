# candev

## Setup Virtual CAN device

```
modprobe vcan
ip link add dev vcan0 type vcan
ip link set up vcan0
```