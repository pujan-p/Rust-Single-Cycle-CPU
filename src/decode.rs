// Decode Instructions

#[derive(Debug)]
pub struct Flags {
    pub opcode: Format,
    pub instruction: u32,
    pub is_valid: bool,
    pub reg_write: bool,
    pub reg_read: bool,
    pub read_mem: bool,
    pub write_mem: bool,
    pub alu_src: bool,
}

impl Flags {

    pub fn reg_write() -> bool {
        false
    }
    
    pub fn reg_read() -> bool {
        true
    }
    
    pub fn read_mem() -> bool {
        false
    }
    
    pub fn write_mem() -> bool {
        false
    }

    pub fn alu_src() -> bool {
        false
    }

}

#[derive(Clone, Debug)]
pub enum Format {
    R,
    I,
    S,
    SB,
    U,
    UJ,
    UNKNOWN
}


#[derive(Debug)]
pub enum AluOP {
    Add,
    Sub,
    AddI,
    SubI,
    Load,
    Store,
    UNKNOWN,
}

pub fn opcode(instruction: &u32) -> Format {
    let mask: u32 = 127;
    let op_val: u32 = (mask & instruction).try_into().unwrap();

    match op_val {
        51 => Format::R,
        19 => Format::I,
        35 => Format::S,
        _ => Format::UNKNOWN
    }
}

pub fn funct7(instruction: &u32) -> (u8, bool) {
    let op: Format = opcode(instruction);
    let mask: u32 = 31;
    match op {
        Format::R => ((mask & (instruction >> 25)).try_into().unwrap(), true),
        _ => (0, false)
    }
}

pub fn read_register_1(instruction: &u32) -> u8 {
    let mask: u32 = 31;
    (mask & (instruction >> 15)).try_into().unwrap()
}

pub fn read_register_2(instruction: &u32) -> u8 {
    let mask: u32 = 31;
    (mask & (instruction >> 20)).try_into().unwrap()
}

pub fn write_register(instruction: &u32) -> u8 {
    let mask: u32 = 31;
    (mask & (instruction >> 7)).try_into().unwrap()
}

pub fn alu_control(instruction: &u32) -> AluOP {
    let f7 = funct7(instruction);
    if f7.1 {
        match f7.0 {
            0 => return AluOP::Add,
            72 => return AluOP::Sub,
            _ => return AluOP::UNKNOWN,
        }
    }
    AluOP::UNKNOWN
}

pub fn write_data() -> u32 {
    0
}