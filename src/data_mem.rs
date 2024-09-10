// Data Memory

#[derive(Debug)]
pub struct DataMemReq {
    pub address: u32,
    pub write_data: u32,
    pub read_mem: bool,
    pub write_mem: bool,
    pub is_valid: bool,
}

#[derive(Debug)]
pub struct DataMemRsp {
    pub read_data: u32,
    pub is_valid: bool,
}

#[derive(Debug)]
pub struct DataMem {
    pub data_mem: [u32; 32],
}

impl DataMem {
    
    pub fn new() -> Self {
        let init_data_mem: [u32; 32] = [0; 32];
        // Todo: Read from FILE
        DataMem { data_mem: init_data_mem }
    }

    pub fn perform_function(&mut self, req: DataMemReq) -> DataMemRsp {
        if !req.is_valid {
            return DataMemRsp {
                read_data: 0,
                is_valid: false,
            }
        }
        
        if req.write_mem {
            self.data_mem[req.address as usize] = req.write_data;
        }

        DataMemRsp {
            read_data: if req.read_mem { self.data_mem[req.address as usize] } else { 0 },
            is_valid: req.read_mem,
        }
    }
}