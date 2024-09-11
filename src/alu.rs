// Alu

use crate::decode::AluOP;
use crate::decode::Flags;

#[derive(Debug)]
pub struct AluReq {
    pub read_data_1: u32, 
    pub read_data_2: u32,
    pub alu_control: AluOP,
    pub flags: Flags,
}

#[derive(Debug)]
pub struct AluRsp {
    pub alu_result: u32,
    pub flags: Flags,
}

pub fn perform_function(req: &AluReq) -> AluRsp {
    
    if !req.flags.is_valid {
        return AluRsp {
            alu_result: 0,
            flags: Flags {
                opcode: req.flags.opcode.clone(),
                ..req.flags
            }
        }
    }

    let alu_result: u32;
    match req.alu_control {
        AluOP::Add => alu_result = req.read_data_1 + req.read_data_2,
        AluOP::Sub => alu_result = req.read_data_1 - req.read_data_2,
        _ => alu_result = 0,
    }

    AluRsp {
        alu_result,
        flags: Flags {
            opcode: req.flags.opcode.clone(),
            ..req.flags
        }
    }
}