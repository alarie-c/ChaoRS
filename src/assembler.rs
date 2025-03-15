use std::{ collections::HashMap, u8 };

use crate::ast::Stmt;

#[repr(u32)]
enum Instr {
    Quit = 0,

    /// `LOADI rd imm`
    Loadi(u8, i16) = 1,

    /// `ADD rd rs1 rs2`
    Add(u8, u8, u8) = 2,
}

impl Instr {
    pub fn encode(self) -> u32 {
        match self {
            Instr::Quit => (0 as u32) << 26,

            Instr::Loadi(rd, imm) =>
                ((Instr::Loadi as u32) << 26) | ((rd as u32) << 22) | ((imm as u32) & 0x3ffff),

            Instr::Add(rd, rs1, rs2) =>
                ((Instr::Add as u32) << 26) |
                    ((rd as u32) << 22) |
                    ((rs1 as u32) << 18) |
                    ((rs2 as u32) << 16),
        }
    }
}

pub struct Assembler<'a> {
    pub output: Vec<i32>,
    input_tree: Vec<Stmt>,
    symbol_table: HashMap<&'a String, u8>,
}

impl<'a> Assembler<'a> {
    fn allocate(&mut self, symbol: &'a String, register: u8) {
        self.symbol_table.insert(symbol, register);
    }

    fn deallocate(&mut self, symbol: &'a String) {
        self.symbol_table.remove(symbol);
    }
}

impl<'a> Assembler<'a> {
    pub fn new(input_tree: Vec<Stmt>) -> Self {
        Assembler {
            output: vec![],
            input_tree,
            symbol_table: HashMap::new(),
        }
    }

    pub fn generate_bytecode(&mut self) {}
}
