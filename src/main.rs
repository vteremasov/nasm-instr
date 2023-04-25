use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

const NO_BIN_ERROR_MSG: &str = "
ERROR: no binary provided.
Usage: nasm-instr <path_to_binary>
";
const NO_FILE_ERROR_MSG: &str = "
ERROR: file not found.
Usage: nasm-instr <path_to_binary>
";


// FIXME operators should be in table
const ASM_OPCODES: [(u8, &str); 2] = [
    (0b100010, "MOV"),
    (0b111111, "PUSH"),
];

const REGISTERS_X: [&str; 8] = [
    "AX",
    "CX",
    "DX",
    "BX",
    "SP",
    "BP",
    "SI",
    "DI",
];
const REGISTERS_L: [&str; 8] = [
    "AL",
    "CL",
    "DL",
    "BL",
    "AH",
    "CH",
    "DH",
    "BH",
];

const REG_LX: [[&str; 8]; 2] = [REGISTERS_L, REGISTERS_X];

fn get_opcode(code: u8) -> Option<String> {
    for (opcode, op) in ASM_OPCODES {
        if code == opcode {
            return Some(String::from(op));
        }
    }

    None
}

fn decompile(bin: Vec<u8>) -> Vec<u8> {
    let mut result = String::from("bits 16

");
    let instr_len = bin.len();
    let mut n = 0;
    while n < instr_len {
        let op = get_opcode(bin[n] >> 2).expect(&format!("Opcode {} not found", bin[n] >> 2));
        let reg_table_idx = (bin[n] | 8) & 1; // Last bit of byte
        let reg_table = REG_LX.get(reg_table_idx as usize).expect(&format!("Regex table for W: {} not found", bin[n] >> 7));
        let md = bin[n+1] >> 6; // First 2 bits of byte
        if md != 0b11 {
            println!("ERROR: MOD 0b11 is supported now. Others not implemented");
            process::exit(1);
        }
        let reg_mem_idx = bin[n+1] & ((1 << 3) - 1); // Last 3 bits of byte
        let reg_idx = (bin[n+1] & ((1 << 6) - 1)) >> 3; // Midle 3 bits of byte
        let reg_mem = String::from(*reg_table.get(reg_mem_idx as usize).expect(&format!("ERROR: register {:x?} not found", reg_mem_idx)));
        let reg = String::from(*reg_table.get(reg_idx as usize).expect(&format!("ERROR: register {:x?} not found", reg_idx)));
        
        result += &format!("{} {}, {}\n", op, reg_mem, reg).to_lowercase();
        n += 2;
    }
    result.try_into().unwrap()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path_to_bin = args.get(1).expect(NO_BIN_ERROR_MSG);
    println!("______________________//_________________________________");
    println!("Decompiling: {}", path_to_bin);
    println!("______________________//_________________________________");
    let mut bin_content: Vec<u8> = Vec::new();
    let mut bin_file = File::open(path_to_bin).expect(&format!("Path: {} {}", path_to_bin, NO_FILE_ERROR_MSG));
    bin_file.read_to_end(&mut bin_content).expect(&format!("ERROR: expect {} to be 8086 instraction set binary", path_to_bin));
    println!("File content: {:?}", bin_content);
    if bin_content.len() % 2 != 0 {
        println!("ERROR: File is not 16 bit");
        process::exit(1);
    }
    let result = decompile(bin_content);
    println!("{:x?}", result);
    let mut dest_file = File::create(&format!("{}test.asm", path_to_bin)).expect("ERROR: Cannot create file in current directory");
    dest_file.write_all(&result).expect("ERROR: Could not write to file");
    dest_file.sync_all().unwrap();
}
