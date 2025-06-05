use rtbf::ir_generator::*;
use rtbf::ir_interpreter::IRInterpreter;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        eprintln!("[ERROR] No program provided !");
        eprintln!("[ERROR] Usage : ./rtbf <my_program.bf>");
        std::process::exit(1);
    }

    let program_path = &args[1];

    let mut program_file = File::open(program_path).expect("[ERROR] Unable to open the file you provided !");

    let mut ir_generator = IRGenerator::new();
    let mut ir_interpreter = IRInterpreter::new();

    let mut program_buffer = String::new();

    program_file.read_to_string(&mut program_buffer).expect("[ERROR] Unable to read the file you provided !");

    let ir_code = ir_generator.generate_ir_from_raw_commands(program_buffer.as_str());

    ir_interpreter.load_ir(ir_code);

    ir_interpreter.interpret_ir();
}
