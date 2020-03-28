use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Dreturn;

impl Instruction for Dreturn {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::dreturn.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 1)
    }
}