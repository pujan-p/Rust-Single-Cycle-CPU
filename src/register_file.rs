// Register File

#[derive(Debug)]
pub struct RegisterFileReq {
    pub read_register_1: usize,
    pub read_register_2: usize,
    pub write_register: usize,
    pub write_data: u32, // !What type?
    pub reg_write: bool
}

#[derive(Debug)]
pub struct RegisterFileRsp {
    pub read_data_1: u32, // !What type?
    pub read_data_2: u32, // !What type?
}

#[derive(Debug)]
pub struct RegisterFile {
    regs: [u32; 32], // !What type?
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        let mut init_regs: [u32; 32] = [0; 32]; // *Can modify individual regs as necessary after
        RegisterFile { regs: init_regs }
    }

    pub fn perform_function(&mut self, req: &RegisterFileReq) -> RegisterFileRsp {
        if req.reg_write != true {
            return RegisterFileRsp {
                read_data_1: self.regs[req.read_register_1],
                read_data_2: self.regs[req.read_register_2],
            }
        } else {
            self.regs[req.write_register] = req.write_data;
        }
        self.regs[0] = 0;
        RegisterFileRsp {
            read_data_1: self.regs[0],
            read_data_2: self.regs[0],
        }
    }
}
