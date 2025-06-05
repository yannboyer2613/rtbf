use crate::lexer::Lexer;

#[derive(Clone, Copy, PartialEq)]
pub enum IRInstructionKind {
    IncrementPointer,
    DecrementPointer,
    IncrementByte,
    DecrementByte,
    PrintByte,
    ReadByte,
    JumpIfZero,
    JumpIfNotZero,
}

#[derive(Clone, Copy, PartialEq)]
pub struct IRInstruction {
    pub kind: IRInstructionKind,
    pub operand: Option<u8>,
}

pub struct IRGenerator {
    lexer: Lexer,
}

impl IRGenerator {
    pub fn new() -> IRGenerator {
        IRGenerator {
            lexer: Lexer::new(),
        }
    }

    pub fn generate_ir_from_raw_commands(&mut self, code_raw: &str) -> Vec<IRInstruction> {
        self.lexer.fill(code_raw);

        let mut c = self.lexer.next();

        let mut temp_program: Vec<IRInstruction> = Vec::new();

        while c != '@' {
            let ir_inst: IRInstruction;
            match c {
                '>' | '<' | '+' | '-' | '.' => {
                    let inst_kind: IRInstructionKind;
                    if c == '>' { inst_kind = IRInstructionKind::IncrementPointer; }
                    else if c == '<' { inst_kind = IRInstructionKind::DecrementPointer; }
                    else if c == '+' { inst_kind = IRInstructionKind::IncrementByte; }
                    else if c == '-' { inst_kind = IRInstructionKind::DecrementByte; }
                    else { inst_kind = IRInstructionKind::PrintByte; }

                    let mut streak = 1 as u8;
                    let mut s = self.lexer.next();

                    while c == s {
                        streak += 1;
                        s = self.lexer.next();
                    }

                    ir_inst = IRInstruction { kind: inst_kind, operand: Some(streak) };

                    c = s;
                },
                ',' | '[' | ']' => {
                    let inst_kind: IRInstructionKind;
                    if c == ',' { inst_kind = IRInstructionKind::ReadByte; }
                    else if c == '[' { inst_kind = IRInstructionKind::JumpIfZero; }
                    else { inst_kind = IRInstructionKind::JumpIfNotZero; }

                    ir_inst = IRInstruction { kind: inst_kind, operand: None };

                    c = self.lexer.next();
                },
                _ => continue,
            }

            temp_program.push(ir_inst);
        }

        return temp_program;
    }
}
