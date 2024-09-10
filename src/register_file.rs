// Register File

#[derive(Debug)]
pub struct RegisterFileReq {
    pub read_register_1: u8,
    pub read_register_2: u8,
    pub write_register: u8,
    pub write_data: u32,
    pub reg_write: bool,
    pub reg_read: bool,
    pub is_valid: bool,
}

#[derive(Debug)]
pub struct RegisterFileRsp {
    pub read_data_1: u32,
    pub read_data_2: u32,
    pub is_valid: bool,
}

#[derive(Debug)]
pub struct RegisterFile {
    regs: [u32; 32],
}

impl RegisterFile {

    pub fn new() -> Self {
        let mut init_regs: [u32; 32] = [0; 32];
        init_regs[1] = 28;
        RegisterFile { regs: init_regs }
    }

    pub fn perform_function(&mut self, req: &RegisterFileReq) -> RegisterFileRsp {

        if !req.is_valid {
            return RegisterFileRsp {
                read_data_1: 0,
                read_data_2: 0,
                is_valid: false
            }
        }

        let rr_1 = req.read_register_1 as usize;
        let rr_2 = req.read_register_2 as usize;
        let wr = req.write_register as usize;
        
        if req.reg_write {
            self.regs[wr] = req.write_data;
        }

        let read_data_1 = if req.reg_read { self.regs[rr_1] } else { 0 };
        let read_data_2 = if req.reg_read { self.regs[rr_2] } else { 0 };
        let is_valid = req.reg_read;

        RegisterFileRsp {
            read_data_1,
            read_data_2,
            is_valid
        }
    }
}
