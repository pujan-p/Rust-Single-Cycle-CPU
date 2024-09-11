// Instruction Memory
// TODO: read from "path/instruction_memory.txt"

use crate::decode::Format;
use crate::decode::Flags;

#[derive(Debug)]
pub struct InstructionMemReq {
    pub read_address: u32,
    pub flags: Flags,
}

#[derive(Debug)]
pub struct InstructionMemRsp {
    pub instruction: u32,
    pub flags: Flags,
}

#[derive(Debug)]
pub struct InstructionMem {
    pub instruction_mem: [u32; 32],
}

impl InstructionMem {

    pub fn new() -> Self {
        let mut init_instruction_mem: [u32; 32] = [0; 32];
        init_instruction_mem[0] = 0x001080b3; // add x1, x1, x1
        // init_instruction_mem[1] = 0x06410113; // addi x2, x2, 100
        // init_instruction_mem[2] = 0x00302823; // sw x3, 16(x0)
        // init_instruction_mem[3] = 0x01002203; // lw x4, 16(x0)

        InstructionMem { instruction_mem: init_instruction_mem }
    }

    pub fn perform_function(&self, req: &InstructionMemReq) -> InstructionMemRsp {
        if !req.flags.is_valid {
            return InstructionMemRsp {
                instruction: 0,
                flags: Flags {
                    opcode: req.flags.opcode.clone(),
                    ..req.flags
                },
            }
        }
        InstructionMemRsp {
            instruction: self.instruction_mem[req.read_address as usize],
            flags: Flags {
                opcode: req.flags.opcode.clone(),
                instruction: self.instruction_mem[req.read_address as usize],
                ..req.flags
            },
        }
    }
}