/*
 * Copyright (c)  2023 Naitik Mundra.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */
use crate::instructions::{NativeTypes, Opcode, Register};
use std::f32;
use std::mem::size_of;

/// We implement a a register based VM.
#[derive(Clone, Debug)]
pub struct VM {
    pub registers: [NativeTypes; 32],
    pub pc: usize, // program counter
    pub program: Vec<u8>,
}

impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}

impl VM {
    fn decode(&mut self) -> Opcode {
        match self.program[self.pc] {
            0 => Opcode::HLT,
            1 => {
                self.pc += 1;
                let reg = self.program[self.pc] as Register;
                let var = self.decode_type();
                Opcode::LOAD(reg, var)
            }
            2 => {
                self.pc += 1;
                let reg1 = self.program[self.pc] as Register;
                self.pc += 1;
                let reg2 = self.program[self.pc] as Register;
                self.pc += 1;
                let reg3 = self.program[self.pc] as Register;
                Opcode::ADD(reg1, reg2, reg3)
            }
            3 => {
                self.pc += 1;
                let reg1 = self.program[self.pc] as Register;
                self.pc += 1;
                let reg2 = self.program[self.pc] as Register;
                self.pc += 1;
                let reg3 = self.program[self.pc] as Register;
                Opcode::SUB(reg1, reg2, reg3)
            }
            4 => {
                self.pc += 1;
                let reg1 = self.program[self.pc] as Register;
                self.pc += 1;
                let reg2 = self.program[self.pc] as Register;
                self.pc += 1;
                let reg3 = self.program[self.pc] as Register;
                Opcode::MUL(reg1, reg2, reg3)
            }
            5 => {
                self.pc += 1;
                let reg1 = self.program[self.pc] as Register;
                self.pc += 1;
                let reg2 = self.program[self.pc] as Register;
                self.pc += 1;
                let reg3 = self.program[self.pc] as Register;
                Opcode::DIV(reg1, reg2, reg3)
            }
            _ => Opcode::IGL,
        }
    }

    fn decode_type(&mut self) -> NativeTypes {
        self.pc += 1;
        match self.program[self.pc] {
            0 => {
                self.pc += 1;
                let size = size_of::<u8>();
                let value = u8::from_le_bytes(self.program[self.pc..][..size].try_into().unwrap());
                self.pc += size;
                NativeTypes::U8(value)
            }
            1 => NativeTypes::I8(self.next_8_bits() as i8),
            2 => NativeTypes::U16(self.next_16_bits()),
            3 => NativeTypes::I16(self.next_16_bits() as i16),
            4 => {
                self.pc += 1;
                let size = size_of::<u32>();
                let value = u32::from_le_bytes(self.program[self.pc..][..size].try_into().unwrap());
                self.pc += size;
                NativeTypes::U32(value)
            }
            5 => {
                self.pc += 1;
                let size = size_of::<i32>();
                let value = i32::from_le_bytes(self.program[self.pc..][..size].try_into().unwrap());
                self.pc += size;
                NativeTypes::I32(value)
            }
            6 => {
                self.pc += 1;
                let size = size_of::<u64>();
                let value = u64::from_le_bytes(self.program[self.pc..][..size].try_into().unwrap());
                self.pc += size;
                NativeTypes::U64(value)
            }
            7 => {
                self.pc += 1;
                let size = size_of::<i64>();
                let value = i64::from_le_bytes(self.program[self.pc..][..size].try_into().unwrap());
                self.pc += size;
                NativeTypes::I64(value)
            }
            8 => {
                self.pc += 1;
                let size = size_of::<f32>();
                let value = f32::from_le_bytes(self.program[self.pc..][..size].try_into().unwrap());
                self.pc += size;
                NativeTypes::F32(value)
            }
            9 => {
                self.pc += 1;
                let size = size_of::<f64>();
                let value = f64::from_le_bytes(self.program[self.pc..][..size].try_into().unwrap());
                self.pc += size;
                NativeTypes::F64(value)
            }
            _ => NativeTypes::None(),
        }
    }

    fn next_8_bits(&mut self) -> u8 {
        self.pc += 1;
        self.program[self.pc]
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc + 1] as u16) << 8) | self.program[self.pc + 2] as u16;
        self.pc += 2;
        result
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() - 1 {
            return true;
        }
        match self.decode() {
            Opcode::LOAD(reg, val) => {
                self.registers[reg] = val;
            }

            Opcode::ADD(reg1, reg2, reg3) => {
                self.registers[reg3] = (self.registers[reg1] + self.registers[reg2]).unwrap();
            }

            Opcode::SUB(reg1, reg2, reg3) => {
                self.registers[reg3] = (self.registers[reg1] - self.registers[reg2]).unwrap();
            }

            Opcode::MUL(reg1, reg2, reg3) => {
                let val1 = self.registers[reg1];
                let val2 = self.registers[reg2];
                match (val1, val2) {
                    (NativeTypes::U8(u), NativeTypes::U8(v)) => {
                        self.registers[reg3] = NativeTypes::U8(u * v)
                    }
                    (NativeTypes::I8(u), NativeTypes::I8(v)) => {
                        self.registers[reg3] = NativeTypes::I8(u * v)
                    }
                    (NativeTypes::U16(u), NativeTypes::U16(v)) => {
                        self.registers[reg3] = NativeTypes::U16(u * v)
                    }
                    (NativeTypes::I16(u), NativeTypes::I16(v)) => {
                        self.registers[reg3] = NativeTypes::I16(u * v)
                    }
                    (NativeTypes::U32(u), NativeTypes::U32(v)) => {
                        self.registers[reg3] = NativeTypes::U32(u * v)
                    }
                    (NativeTypes::I32(u), NativeTypes::I32(v)) => {
                        self.registers[reg3] = NativeTypes::I32(u * v)
                    }
                    (NativeTypes::U64(u), NativeTypes::U64(v)) => {
                        self.registers[reg3] = NativeTypes::U64(u * v)
                    }
                    (NativeTypes::I64(u), NativeTypes::I64(v)) => {
                        self.registers[reg3] = NativeTypes::I64(u * v)
                    }
                    (NativeTypes::F32(u), NativeTypes::F32(v)) => {
                        self.registers[reg3] = NativeTypes::F32(u * v)
                    }
                    (NativeTypes::F64(u), NativeTypes::F64(v)) => {
                        self.registers[reg3] = NativeTypes::F64(u * v)
                    }
                    _ => return true,
                }
            }

            Opcode::DIV(reg1, reg2, reg3) => {
                let val1 = self.registers[reg1];
                let val2 = self.registers[reg2];
                match (val1, val2) {
                    (NativeTypes::U8(u), NativeTypes::U8(v)) => {
                        self.registers[reg3] = NativeTypes::U8(u / v)
                    }
                    (NativeTypes::I8(u), NativeTypes::I8(v)) => {
                        self.registers[reg3] = NativeTypes::I8(u / v)
                    }
                    (NativeTypes::U16(u), NativeTypes::U16(v)) => {
                        self.registers[reg3] = NativeTypes::U16(u / v)
                    }
                    (NativeTypes::I16(u), NativeTypes::I16(v)) => {
                        self.registers[reg3] = NativeTypes::I16(u / v)
                    }
                    (NativeTypes::U32(u), NativeTypes::U32(v)) => {
                        self.registers[reg3] = NativeTypes::U32(u / v)
                    }
                    (NativeTypes::I32(u), NativeTypes::I32(v)) => {
                        self.registers[reg3] = NativeTypes::I32(u / v)
                    }
                    (NativeTypes::U64(u), NativeTypes::U64(v)) => {
                        self.registers[reg3] = NativeTypes::U64(u / v)
                    }
                    (NativeTypes::I64(u), NativeTypes::I64(v)) => {
                        self.registers[reg3] = NativeTypes::I64(u / v)
                    }
                    (NativeTypes::F32(u), NativeTypes::F32(v)) => {
                        self.registers[reg3] = NativeTypes::F32(u / v)
                    }
                    (NativeTypes::F64(u), NativeTypes::F64(v)) => {
                        self.registers[reg3] = NativeTypes::F64(u / v)
                    }
                    _ => return true,
                }
            }

            Opcode::HLT => {
                println!("HLT encountered");
                self.pc += 1;
                return true;
            }

            Opcode::IGL => {
                println!("IGL encountered at {}", self.pc);
                self.pc += 1;
                return true;
            }
        }
        false
    }

    pub fn new() -> VM {
        VM {
            registers: [NativeTypes::U8(0); 32],
            program: vec![],
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], NativeTypes::U8(0))
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1)
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![255, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1)
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 5, 248, 255, 255, 255, 1, 1, 5, 8, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[0], NativeTypes::I32(-8));
        assert_eq!(test_vm.registers[1], NativeTypes::I32(8))
    }

    #[test]
    fn test_add() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 5, 248, 255, 255, 255, 1, 1, 5, 8, 0, 0, 0, 2, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], NativeTypes::I32(0))
    }

    #[test]
    fn test_sub() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 5, 248, 255, 255, 255, 1, 1, 5, 8, 0, 0, 0, 3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], NativeTypes::I32(-16))
    }

    #[test]
    fn test_mul() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 8, 0, 0, 188, 65, 1, 1, 8, 236, 81, 9, 66, 4, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], NativeTypes::F32(806.75507))
    }

    #[test]
    fn test_div() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 5, 248, 255, 255, 255, 1, 1, 5, 8, 0, 0, 0, 5, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], NativeTypes::I32(-1))
    }
}
