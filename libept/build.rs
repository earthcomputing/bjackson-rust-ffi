/*---------------------------------------------------------------------------------------------
 *  Copyright © 2016-present Earth Computing Corporation. All rights reserved.
 *  Licensed under the MIT License. See LICENSE.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

use std::env::var;

fn main() {
    let ws_dir = var("WS").expect("WS undefined");
    println!(r"cargo:rustc-link-search={}/bjackson-rust-ffi/libept/src", ws_dir);
    println!(r"cargo:rustc-link-search={}/bjackson-ecnl/lib", ws_dir);
}
