use super::vm_parser::{CommandType, Parser};
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CodeWriter {
    filename: Vec<String>,
    jump_eq: usize,
    jump_gt: usize,
    jump_lt: usize,
    file_index: usize,
}

impl CodeWriter {
    pub fn new() -> CodeWriter {
        CodeWriter {
            filename: Vec::new(),
            jump_eq: 0,
            jump_gt: 0,
            jump_lt: 0,
            file_index: 0,
        }
    }
    pub fn set_filename(&mut self, filename: String) {
        self.filename.push(filename);
    }
    pub fn write_arithmetic(&mut self, command: &mut Parser) -> String {
        match command.arg1().as_str() {
            "add" => "@SP\nM=M-1\nD=M\nA=D\nD=M\nA=A-1\nM=M+D\n".to_string(),
            "sub" => "@SP\nM=M-1\nD=M\nA=D\nD=M\nA=A-1\nM=M-D\n".to_string(),
            "neg" => "@SP\nD=M\nA=D\nD=M\nA=A-1\nM=-M\n".to_string(),
            "eq" => {
                self.jump_eq += 1;
                format!("@SP\nM=M-1\nD=M\nA=D\nD=M\nA=A-1\nD=M-D\n@EQ_JUMP{0}\nD;JEQ\n@SP\nD=M\nA=D-1\nM=0\n@EQ_END{0}\n0;JMP\n(EQ_JUMP{0})\n@SP\nD=M\nA=D-1\nM=-1\n(EQ_END{0})\n", self.jump_eq - 1)
            }
            "gt" => {
                self.jump_gt += 1;
                format!("@SP\nM=M-1\nD=M\nA=D\nD=M\nA=A-1\nD=M-D\n@GT_JUMP{0}\nD;JGT\n@SP\nD=M\nA=D-1\nM=0\n@GT_END{0}\n0;JMP\n(GT_JUMP{0})\n@SP\nD=M\nA=D-1\nM=-1\n(GT_END{0})\n", self.jump_gt - 1)
            }
            "lt" => {
                self.jump_lt += 1;
                format!("@SP\nM=M-1\nD=M\nA=D\nD=M\nA=A-1\nD=M-D\n@LT_JUMP{0}\nD;JLT\n@SP\nD=M\nA=D-1\nM=0\n@LT_END{0}\n0;JMP\n(LT_JUMP{0})\n@SP\nD=M\nA=D-1\nM=-1\n(LT_END{0})\n", self.jump_lt - 1)
            }
            "and" => "@SP\nM=M-1\nD=M\nA=D\nD=M\nA=A-1\nM=M&D\n".to_string(),
            "or" => "@SP\nM=M-1\nD=M\nA=D\nD=M\nA=A-1\nM=M|D\n".to_string(),
            "not" => "@SP\nD=M\nA=D\nD=M\nA=A-1\nM=!M\n".to_string(),
            _ => "".to_string(),
        }
    }
    pub fn write_push_pop(&mut self, command: &mut Parser) -> String {
        if command.command_type() == CommandType::C_PUSH {
            match command.command[1].as_str() {
                "constant" => format!("@{0}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", command.arg2()),
                "local" => format!(
                    "@{0}\nD=A\n@LCL\nA=M\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
                    command.arg2()
                ),
                "argument" => format!(
                    "@{0}\nD=A\n@ARG\nA=M\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
                    command.arg2()
                ),
                "this" => format!(
                    "@{0}\nD=A\n@THIS\nA=M\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
                    command.arg2()
                ),
                "that" => format!(
                    "@{0}\nD=A\n@THAT\nA=M\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
                    command.arg2()
                ),
                "pointer" => format!(
                    "@{0}\nD=A\n@R3\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
                    command.arg2()
                ),
                "temp" => format!(
                    "@{0}\nD=A\n@R5\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
                    command.arg2()
                ),
                "static" => format!(
                    "@{0}.{1}\nD=M\n@SP\nA=M\nM=D\n\n@SP\nM=M+1\n",
                    self.filename[self.file_index].replace(".vm", ""),
                    command.arg2()
                ),
                _ => "".to_string(),
            }
        } else {
            match command.command[1].as_str() {
                "local" => format!(
                    "@{0}\nD=A\n@LCL\nM=M+D\n@{0}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@LCL\nA=M\nM=D\n@{0}\nD=A\n@LCL\nM=M-D\n",
                    command.arg2()
                ),
                "argument" => format!(
                    "@{0}\nD=A\n@ARG\nM=M+D\n@{0}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@ARG\nA=M\nM=D\n@{0}\nD=A\n@ARG\nM=M-D\n",
                    command.arg2()
                ),
                "this" => format!(
                    "@{0}\nD=A\n@THIS\nM=M+D\n@{0}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@THIS\nA=M\nM=D\n@{0}\nD=A\n@THIS\nM=M-D\n",
                    command.arg2()
                ),
                "that" => format!(
                    "@{0}\nD=A\n@THAT\nM=M+D\n@{0}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@THAT\nA=M\nM=D\n@{0}\nD=A\n@THAT\nM=M-D\n",
                    command.arg2()
                ),
                "pointer" => format!(
                    "@SP\nM=M-1\nD=M\nA=D\nD=M\n@R3\n{}M=D\n",
                    "A=A+1\n".to_string().repeat(command.arg2() as usize)
                ),
                "temp" => format!(
                    "@SP\nM=M-1\nD=M\nA=D\nD=M\n@R5\n{}M=D\n",
                    "A=A+1\n".to_string().repeat(command.arg2() as usize)
                ),
                "static" => format!(
                    "@SP\nM=M-1\nD=M\nA=D\nD=M\n@{0}.{1}\nM=D\n",
                    self.filename[self.file_index].replace(".vm", ""),
                    command.arg2()
                ),
                _ => "".to_string()
            }
        }
    }
    pub fn write_file(&mut self, dir_name: &str, command: &mut Parser) {
        let outfile = dir_name.to_string() + ".asm";
        let mut write_str: String = String::new();
        while command.advance() {
            if command.command_type() == CommandType::C_ARITHMETIC {
                write_str += &self.write_arithmetic(command);
            } else if command.command_type() == CommandType::C_PUSH
                || command.command_type() == CommandType::C_POP
            {
                write_str += &self.write_push_pop(command);
            } else if command.command_type() == CommandType::FILE_START {
                self.file_index += 1;
            }
        }
        let mut f = BufWriter::new(File::create(outfile).unwrap());
        write!(f, "{}", write_str).unwrap();
    }
}
