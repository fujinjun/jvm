#![allow(non_camel_case_types)]
use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Dstore_1;

impl Instruction for Dstore_1 {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::dstore_1.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 1)
    }
}