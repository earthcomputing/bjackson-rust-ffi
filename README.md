# bjackson-rust-ffi

## libept

## testing

    sudo insmod ${WS}/bjackson-e1000e/e1000e-3.3.4/src/e1000e.ko
    sudo insmod ${WS}/bjackson-ecnl/src/ecnl_device.ko

    ( cd libept/src ; make )
    cd libept
    env RUSTFLAGS="-C debuginfo=2 -A dead_code -A unused-variables -A unused-imports -A non-snake-case" cargo test --release

    cargo test --release -- --nocapture

## valgrind

    sudo valgrind --leak-check=full target/release/deps/libept-* --nocapture

