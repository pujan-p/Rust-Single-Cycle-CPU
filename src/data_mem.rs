// Data Memory

use crate::decode::Flags;

#[derive(Debug)]
pub struct DataMemReq {
    pub address: u32,
    pub write_data: u32,
    pub flags: Flags,
}

#[derive(Debug)]
pub struct DataMemRsp {
    pub read_data: u32,
    pub flags: Flags,
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

    pub fn perform_function(&mut self, req: &DataMemReq) -> DataMemRsp {
        if !req.flags.is_valid {
            return DataMemRsp {
                read_data: 0,
                flags: Flags {
                    opcode: req.flags.opcode.clone(),
                    ..req.flags
                }
            }
        }
        
        if req.flags.write_mem {
            self.data_mem[req.address as usize] = req.write_data;
        }

        DataMemRsp {
            read_data: if req.flags.read_mem { self.data_mem[req.address as usize] } else { 0 },
            
            flags: Flags {
                opcode: req.flags.opcode.clone(),
                ..req.flags
            }
        }
    }
}