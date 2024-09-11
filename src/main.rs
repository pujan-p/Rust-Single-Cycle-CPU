use decode::Flags;

mod instruction_mem;
mod decode;
mod register_file;
mod alu;
mod data_mem;

fn main() {
    // *Instruction Memory:
    let instr_mem_req = instruction_mem::InstructionMemReq {
        read_address: 0,
        flags: Flags {
            opcode: decode::Format::UNKNOWN,
            instruction: 0,
            is_valid: true,
            reg_write: Flags::reg_write(),
            reg_read: Flags::reg_read(),
            read_mem: Flags::read_mem(),
            write_mem: Flags::write_mem(),
            alu_src: Flags::alu_src(),
        }
    };
    let instr_mem = instruction_mem::InstructionMem::new();
    let instr_mem_rsp = instr_mem.perform_function(&instr_mem_req);

    println!("instr_mem_req: {instr_mem_req:#?}");
    println!("instr_mem_rsp: {instr_mem_rsp:#?}");

    // *Register File:
    let reg_file_req = register_file::RegisterFileReq {
        read_register_1: decode::read_register_1(&instr_mem_rsp.instruction),
        read_register_2: decode::read_register_2(&instr_mem_rsp.instruction),
        write_register: decode::write_register(&instr_mem_rsp.instruction),
        write_data: 0,
        flags: Flags{
            opcode: decode::opcode(&instr_mem_rsp.instruction),
            ..instr_mem_rsp.flags
        },
    };
    let mut reg_file = register_file::RegisterFile::new();
    let reg_file_rsp = reg_file.perform_function(&reg_file_req);

    println!("reg_file_req: {reg_file_req:#?}");
    println!("reg_file_rsp: {reg_file_rsp:#?}");

    // *Intermediate Generator:

    // *ALU:
    let alu_req = alu::AluReq {
        read_data_1: reg_file_rsp.read_data_1,
        read_data_2: if !reg_file_rsp.flags.alu_src { reg_file_rsp.read_data_2 } else { 0 },
        alu_control: decode::alu_control(&reg_file_rsp.flags.instruction),
        flags: Flags {
            ..reg_file_rsp.flags
        }
    };
    let alu_rsp = alu::perform_function(&alu_req);

    println!("alu_req: {alu_req:#?}");
    println!("alu_rsp: {alu_rsp:#?}");

    // *Data Memory:
    let data_mem_req = data_mem::DataMemReq {
        address: alu_rsp.alu_result,
        write_data: reg_file_rsp.read_data_2,
        flags: Flags {
            ..alu_rsp.flags
        }
    };
    let mut data_mem = data_mem::DataMem::new();
    let data_mem_rsp = data_mem.perform_function(&data_mem_req);

    println!("data_mem_req: {data_mem_req:#?}");
    println!("data_mem_rsp: {data_mem_rsp:#?}");

    // *Write Back:
    let reg_file_req_2 = register_file::RegisterFileReq {
        read_register_1: 0,
        read_register_2: 0,
        write_register: decode::write_register(&data_mem_rsp.flags.instruction),
        write_data: alu_rsp.alu_result,
        flags: Flags{
            opcode: decode::opcode(&data_mem_rsp.flags.instruction),
            reg_write: true,
            ..data_mem_rsp.flags
        },
    };
    let reg_file_rsp_2 = reg_file.perform_function(&reg_file_req_2);

    println!("data_mem_req: {reg_file_req_2:#?}");
    println!("data_mem_rsp: {reg_file_rsp_2:#?}"); 
}