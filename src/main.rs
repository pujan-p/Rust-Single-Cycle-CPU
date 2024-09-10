use std::sync::Arc;

pub mod register_file;
pub mod alu;

fn main() {

    let alu_req = alu::AluReq {
        read_data_1: 0b110,
        read_data_2: 0b110,
        alu_control: alu::AluOP::Add,
        is_valid: true
    };

    let alu_rsp = alu::perform_function(&alu_req);

    println!("Alu_Rsp: {alu_rsp:#?}")
}