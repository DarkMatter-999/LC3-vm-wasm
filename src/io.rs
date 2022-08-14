use std::io::{Read, self, Write};

use super::*;

#[link(wasm_import_module = "../utils.js")]
extern { fn printlog(data: char); }

#[link(wasm_import_module = "../utils.js")]
extern { fn printdisassembly(data: char); }

#[link(wasm_import_module = "../utils.js")]
extern { fn getkey() -> u8; }

#[link(wasm_import_module = "../utils.js")]
extern { fn printpc(pc: usize); }

pub fn get_key() -> u8 {
    let key = unsafe { getkey() };

    unsafe { printlog(key as char) }

    return key;
}

pub fn print(data: u8) {
    unsafe { printlog(data as char) }
}

pub fn printstr(string: String) {
    for s in string.chars() {
        unsafe { printdisassembly(s) }
    }
    unsafe { printdisassembly('\n') }
}

pub fn pushpc(pc: usize) {
    unsafe { printpc(pc) }
}