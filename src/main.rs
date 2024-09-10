mod instruction_mem;
mod decode;
mod register_file;
mod alu;
mod data_mem;

fn main() {

    // Instruction Memory:
    let instr_mem_req = instruction_mem::InstructionMemReq {
        read_address: 0,
        is_valid: true,
    };
    let instr_mem = instruction_mem::InstructionMem::new();
    let instr_mem_rsp = instr_mem.perform_function(&instr_mem_req);

    // Register File:
    let reg_file_req = register_file::RegisterFileReq {
        read_register_1: 0b0000_1,
        read_register_2: 0b0000_1,
        write_register: 0b0000_1,
        write_data: 0,
        reg_write: false,
        reg_read: true,
        is_valid: true,
    };
    let mut reg_file = register_file::RegisterFile::new();
    let reg_file_rsp = reg_file.perform_function(&reg_file_req);

    // ALU:
    let alu_req = alu::AluReq {
        read_data_1: reg_file_rsp.read_data_1,
        read_data_2: reg_file_rsp.read_data_2,
        alu_control: alu::AluOP::Add,
        is_valid: true,
    };
    let alu_rsp = alu::perform_function(&alu_req);


    // Data Memory:
    let data_mem_req = data_mem::DataMemReq {
        address: alu_rsp.alu_result,
        write_data: reg_file_rsp.read_data_2,
        read_mem: false,
        write_mem: false,
        is_valid: true,
    };
    let mut data_mem = data_mem::DataMem::new();
    let data_mem_rsp = data_mem.perform_function(data_mem_req);


    // Testing:
    println!("instr_mem_rsp: {instr_mem_rsp:#?}");

    println!("reg_file_rsp: {reg_file_rsp:#?}");

    println!("alu_rsp: {alu_rsp:#?}");
    
    println!("data_mem_rsp: {data_mem_rsp:#?}");

}