pub struct Lexer {
    pos_in_code: usize,
    code: Vec<char>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            pos_in_code: 0,
            code: Vec::new(),
        }
    }

    pub fn fill(&mut self, code: &str) {
        for c in code.chars() {
            self.code.push(c);
        }
    }

    fn is_valid_brainfuck_instruction(&self, instruction: char) -> bool {
        let valid_insts = "><+-.,[]";
        if valid_insts.contains(instruction) {
            return true;
        } else {
            return false;
        }
    }

    pub fn next(&mut self) -> char {
        while self.pos_in_code < self.code.len() && !self.is_valid_brainfuck_instruction(self.code[self.pos_in_code]) {
            self.pos_in_code += 1;
        }

        if self.pos_in_code >= self.code.len() {
            return '@'; // TODO: Find a way to better handle EOF.
        }

        let r = self.code[self.pos_in_code];
        self.pos_in_code += 1;
        return r;
    }
}
