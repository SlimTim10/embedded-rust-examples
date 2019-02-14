# Embedded Rust Examples on STM32F3DISCOVERY

## Compiling

## Flash Programming

```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg -c "program filename.elf verify reset exit"
```

## Debugging

```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```