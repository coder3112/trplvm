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
use derive_more::{Add, Sub};

pub type Register = usize;

/// Types implemented directly in the register. These are all numbers. Strings etc will be
/// implemented on the stack or heap depending.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Add, Sub)]
pub enum NativeTypes {
    None(),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Opcode {
    HLT, // Halt
    IGL, // Illegal
    LOAD(Register, NativeTypes),
    ADD(Register, Register, Register),
    SUB(Register, Register, Register),
    MUL(Register, Register, Register),
    DIV(Register, Register, Register),
}

/// An instruction is 32 bits. 8 bit Opcode and 24 bits for 0,1,2 or 3 operands.
#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT)
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT)
    }
}
