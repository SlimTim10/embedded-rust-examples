# Embedded Rust Examples on STM32F3DISCOVERY

## Compiling

Download the Cortex-M minimal startup and runtime:
```
rustup target add thumbv7em-none-eabihf
```

In an example src directory:
```
cargo build --target thumbv7em-none-eabihf
```

## Flash Programming

```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg -c "program target/thumbv7em-none-eabihf/debug/led-pattern verify reset exit"
```

## Debugging

Start OpenOCD server:
```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

Start GDB:

```
arm-none-eabi-gdb -x openocd.gdb -q target/thumbv7em-none-eabihf/debug/led-pattern
```