cargo rustc -- -C link-arg=--script=./linker.ld
# if linker script is modified rm -rf target/
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/pico-rust out/kernel7.img
