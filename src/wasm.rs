use crate::*;
use crate::{cpu, memory::Mem, io::*};

static mut cpu: cpu::CPU = cpu::CPU {
    rr0 : 0,
    rr1 : 0,
    rr2 : 0,
    rr3 : 0,
    rr4 : 0,
    rr5 : 0,
    rr6 : 0,
    rr7 : 0,
    pc : 0x3000,
    rcond : 0,
    rcount : 0,
    memory : Mem {
        memory: [0; 1 << 16],
    },
    running: true
};

#[wasm_bindgen]
pub extern fn loadimage(path: Vec<u8>) {   
    unsafe {
        cpu.load_image(&path)
    };
}

#[wasm_bindgen]
pub extern fn step() {
    unsafe {
        cpu.step()
    };
}