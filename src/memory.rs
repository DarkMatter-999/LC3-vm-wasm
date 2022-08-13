use std::io::Read;

use crate::io::get_key;
const memory_max: usize = 1 << 16;

pub enum MemoryMappedReg {
    // Keyboard status
    kbsr = 0xFE00,

    // Keyboard data
    kbdr = 0xFE02,
}

pub struct Mem {
    pub memory: [u16; memory_max]
}

impl Mem {
    pub fn new() -> Mem {
        Mem { 
            memory: [0; memory_max]
        }
    }

    pub fn read(&mut self, addr: usize) -> u16 {
        if addr == MemoryMappedReg::kbsr as usize {
            self.keyboard();
        }
        self.memory[addr]
    }

    pub fn write(&mut self, addr: usize, val: u16) {
        self.memory[addr] = val;
    }

    fn keyboard(&mut self) {
        let ch = get_key();

        if ch != 0 {
            self.write(MemoryMappedReg::kbsr as usize, 1 << 15);
            self.write(MemoryMappedReg::kbdr as usize, ch as u16);
        } else {
            self.write(MemoryMappedReg::kbsr as usize, 0)
        }
    }
}