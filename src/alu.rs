// Alu

#[derive(Debug)]
pub struct AluReq {
    pub read_data_1: u32, 
    pub read_data_2: u32,
    pub alu_control: AluOP,
    pub is_valid: bool,
}

#[derive(Debug)]
pub struct AluRsp {
    pub alu_result: u32,
    pub is_valid: bool,
}

#[derive(Debug)]
pub enum AluOP {
    Add,
    Sub,
}

pub fn perform_function(req: &AluReq) -> AluRsp {
    
    if !req.is_valid {
        return AluRsp {
            alu_result: 0,
            is_valid: false,
        }
    }
    
    let alu_result: u32;
    match req.alu_control {
        AluOP::Add => alu_result = req.read_data_1 + req.read_data_2,
        AluOP::Sub => alu_result = req.read_data_1 - req.read_data_2,
    }

    AluRsp {
        alu_result,
        is_valid: true
    }
}