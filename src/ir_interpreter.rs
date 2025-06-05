use crate::ir_generator::{IRInstruction, IRInstructionKind};
use std::io::{self, Read, Write};

const TOTAL_MEMORY_CELLS: usize = 100000;

pub struct IRInterpreter {
    ir_program: Vec<IRInstruction>,
    memory: [u8; TOTAL_MEMORY_CELLS],
    instruction_pointer: usize,
    memory_pointer: usize,
}

impl IRInterpreter {
    pub fn new() -> IRInterpreter {
        IRInterpreter {
            ir_program: Vec::new(),
            memory: [0; TOTAL_MEMORY_CELLS],
            instruction_pointer: 0,
            memory_pointer: 0,
        }
    }

    // TODO: Maybe I shouldn't abuse pass by value.
    pub fn load_ir(&mut self, ir_code: Vec<IRInstruction>) {
        self.ir_program = ir_code;
    }

    pub fn interpret_ir(&mut self) {
        let prg_len = self.ir_program.len();
        let mut stack: Vec<usize> = Vec::new();
        let mut targets: Vec<usize> = vec![0; prg_len]; // Dumb workaround because Rust doesn't allow arrays with non const size.

        // Precomputing jumps.
        // HashMap are slow, so I got rid of them.
        let mut j: usize;
        for i in 0..prg_len {
            if self.ir_program[i].kind == IRInstructionKind::JumpIfZero {
                stack.push(i);
            } else if self.ir_program[i].kind == IRInstructionKind::JumpIfNotZero {
                if stack.is_empty() {
                    panic!("[ERROR] Unmatched ']' at byte {}", i + 1);
                } else {
                    j = stack.pop().unwrap();
                    targets[i] = j;
                    targets[j] = i;
                }
            }
        }

        while self.instruction_pointer < prg_len {
            let ir_inst = self.ir_program[self.instruction_pointer];
            let curr_mem_cell_by_ref = &mut self.memory[self.memory_pointer];

            match ir_inst.kind {
                IRInstructionKind::IncrementPointer => self.memory_pointer += ir_inst.operand.unwrap() as usize,
                IRInstructionKind::DecrementPointer => self.memory_pointer -= ir_inst.operand.unwrap() as usize,
                IRInstructionKind::IncrementByte => *curr_mem_cell_by_ref += ir_inst.operand.unwrap(),
                IRInstructionKind::DecrementByte => *curr_mem_cell_by_ref -= ir_inst.operand.unwrap(),
                IRInstructionKind::PrintByte => {
                    let count = ir_inst.operand.unwrap();
                    let byte_as_char = *curr_mem_cell_by_ref as char;

                    for _ in 0..count {
                        print!("{byte_as_char}");
                        io::stdout().flush().unwrap();
                    }
                },
                IRInstructionKind::ReadByte => {
                    let mut input: [u8; 1] = [0; 1];
                    io::stdin().read_exact(&mut input).expect("[ERROR] Unable to read stdin.");
                    self.memory[self.memory_pointer] = input[0];
                },
                IRInstructionKind::JumpIfZero => {
                    if *curr_mem_cell_by_ref == 0 {
                        self.instruction_pointer = targets[self.instruction_pointer];
                    }
                },
                IRInstructionKind::JumpIfNotZero => {
                    if *curr_mem_cell_by_ref != 0 {
                        self.instruction_pointer = targets[self.instruction_pointer];
                    }
                }
            }

            self.instruction_pointer += 1;
        }
    }
}
