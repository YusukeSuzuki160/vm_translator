use std::error::Error;
use std::fs;
use std::path;

enum CommandType {
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
    NULL,
}

struct Parser {
    vm_code: Vec<String>,
    command: String,
    command_type: CommandType,
    now_line: usize,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            vm_code: Vec::new(),
            command: String::new(),
            command_type: CommandType::NULL,
            now_line: 0,
        }
    }
    pub fn load_file(&mut self, filename: &str) {
        let v = fs::read_to_string(filename).unwrap();
        for lines in v.lines() {
            self.vm_code.push(
                lines.to_string().split('/').collect::<Vec<&str>>()[0]
                    .replace(" ", "")
                    .to_string(),
            );
        }
    }
    fn has_more_commands(&self) -> bool {
        if self.now_line == self.vm_code.len() {
            false
        } else {
            true
        }
    }
}
