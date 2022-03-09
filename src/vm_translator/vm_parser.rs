use std::fs;

#[derive(Clone, PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
pub enum CommandType {
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
    FILE_START,
    NULL,
}

pub struct Parser {
    pub vm_code: Vec<String>,
    pub command: Vec<String>,
    c_type: CommandType,
    now_line: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            vm_code: Vec::new(),
            command: Vec::new(),
            c_type: CommandType::NULL,
            now_line: 0,
        }
    }
    pub fn load_file(&mut self, filename: String) {
        let v = fs::read_to_string(filename).unwrap();
        for lines in v.lines() {
            self.vm_code
                .push(lines.split('/').collect::<Vec<&str>>()[0].to_string());
        }
    }
    pub fn command_type(&self) -> CommandType {
        self.c_type.clone()
    }
    fn has_more_commands(&self) -> bool {
        if self.now_line == self.vm_code.len() {
            false
        } else {
            true
        }
    }
    fn load_command(&mut self) {
        let command: Vec<&str> = self.vm_code[self.now_line].split(" ").collect();
        self.command = Vec::new();
        for v in &command {
            self.command.push(
                v.replace(" ", "")
                    .replace("\n", "")
                    .replace("\t", "")
                    .to_string(),
            );
        }
        self.now_line += 1;
        self.c_type = match self.command[0].as_str() {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                CommandType::C_ARITHMETIC
            }
            "push" => CommandType::C_PUSH,
            "pop" => CommandType::C_POP,
            "label" => CommandType::C_LABEL,
            "goto" => CommandType::C_GOTO,
            "if-goto" => CommandType::C_IF,
            "function" => CommandType::C_FUNCTION,
            "return" => CommandType::C_RETURN,
            "call" => CommandType::C_CALL,
            "start" => CommandType::FILE_START,
            _ => CommandType::NULL,
        }
    }
    pub(crate) fn advance(&mut self) -> bool {
        if self.has_more_commands() {
            self.load_command();
            true
        } else {
            false
        }
    }
    pub(crate) fn arg1(&self) -> String {
        //Not C_RETURN.
        if self.command_type() == CommandType::C_ARITHMETIC {
            self.command[0].clone()
        } else {
            self.command[1].clone()
        }
    }
    pub(crate) fn arg2(&self) -> i32 {
        //C_PUSH, C_POP, C_FUNCTION, C_CALL only.
        self.command[2].parse().unwrap()
    }
}
