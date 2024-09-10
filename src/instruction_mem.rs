// Instruction Memory

#[derive(Debug)]
pub struct InstructionMemReq {
    pub read_address: u32,
    pub is_valid: bool,
}

#[derive(Debug)]
pub struct InstructionMemRsp {
    pub instruction: u32,
    pub is_valid: bool,
}

#[derive(Debug)]
pub struct InstructionMem {
    pub instruction_mem: [u32; 32],
}

impl InstructionMem {

    pub fn new() -> Self {
        let mut init_instruction_mem: [u32; 32] = [0; 32];
        init_instruction_mem[0] = 0x001080b3;
        // Todo: Read from FILE
        InstructionMem { instruction_mem: init_instruction_mem }
    }

    pub fn perform_function(&self, req: &InstructionMemReq) -> InstructionMemRsp {
        if !req.is_valid {
            return InstructionMemRsp {
                instruction: 0,
                is_valid: false,
            }
        }
        InstructionMemRsp {
            instruction: self.instruction_mem[req.read_address as usize],
            is_valid: true,
        }
    }
}