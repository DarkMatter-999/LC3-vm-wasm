use std::io::{Read, self, Write};

use super::*;

#[link(wasm_import_module = "../utils.js")]
extern { fn printlog(data: char); }

#[link(wasm_import_module = "../utils.js")]
extern { fn getkey() -> u8; }

pub fn get_key() -> u8 {
    let key = unsafe { getkey() };

    unsafe { printlog(key as char) }

    return key;
}

pub fn print(data: u8) {
    unsafe { printlog(data as char) }
}