
use std::env::var;

fn main() {
    let ws_dir = var("WS").expect("WS undefined");
    println!(r"cargo:rustc-link-search={}/bjackson-rust-ffi/libept/src", ws_dir);
    println!(r"cargo:rustc-link-search={}/bjackson-ecnl/lib", ws_dir);
}
