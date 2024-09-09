pub mod register_file;

fn main() {

    let mut regs  = register_file::RegisterFile::new();

    let mut reg_req = register_file::RegisterFileReq {
        read_register_1: 0,
        read_register_2: 0,
        write_register: 0,
        write_data: 0,
        reg_write: false,
    };

    let mut reg_rsp = register_file::RegisterFile::perform_function(&mut regs, &reg_req);

    println!("reg_rsp is {reg_rsp:#?}");
}