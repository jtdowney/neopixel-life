# Neopixel Game of Life

I picked up some [16x16 neopixel panels from AliExpress](https://www.aliexpress.com/item/32789295872.html) and have wanted to play more with Rust and the STM32F103 development board I have.

## Flashing the code

The STM32 dev board I have includes an STLink on board. Given how sensitive the neopixel driver is to timing, I have only been able to get release builds to display correctly.

```
cargo build --release
openocd -f openocd.cfg -c "program target/thumbv7m-none-eabi/release/neopixel-life; reset; exit"
```

## Result

[![Video](http://img.youtube.com/vi/lah8swbpIZs/0.jpg)](http://www.youtube.com/watch?v=lah8swbpIZs)
