use std::io::{Read, self, Write};

use crate::cpu::CPU;

use super::*;

#[link(wasm_import_module = "../utils.js")]
extern { fn printlog(data: char); }

#[link(wasm_import_module = "../utils.js")]
extern { fn printdisassembly(data: char); }

#[link(wasm_import_module = "../utils.js")]
extern { fn getkey() -> u8; }

#[link(wasm_import_module = "../utils.js")]
extern { fn printpc(pc: usize); }

#[link(wasm_import_module = "../utils.js")]
extern { fn printreg(number: u32, value: u16); }


pub fn get_key() -> u8 {
    let key = unsafe { getkey() };
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

pub fn pushreg(cpu: &CPU) {
    pushr(0,cpu.rr0);
    pushr(1, cpu.rr1);
    pushr(2, cpu.rr2);
    pushr(3, cpu.rr3);
    pushr(4, cpu.rr4);
    pushr(5, cpu.rr5);
    pushr(6, cpu.rr6);
    pushr(7, cpu.rr7);

    pushr(8, cpu.pc as u16);
    pushr(9, cpu.rcond);
}

fn pushr(number: u32, value: u16) {
    unsafe { printreg(number, value) };
}