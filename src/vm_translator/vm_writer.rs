use super::vm_parser::{CommandType, Parser};
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CodeWriter {
    filename: Vec<String>,
    jump_eq: usize,
    jump_gt: usize,
    jump_lt: usize,
    file_index: usize,
    function_name: Vec<String>,
    func_num: usize,
}

impl CodeWriter {
    pub fn new() -> CodeWriter {
        CodeWriter {
            filename: Vec::new(),
            jump_eq: 0,
            jump_gt: 0,
            jump_lt: 0,
            file_index: 0,
            function_name: Vec::new(),
            func_num: 0,
        }
    }
    pub fn set_filename(&mut self, filename: String) {
        self.filename.push(filename);
    }
    fn write_arithmetic(&mut self, command: &mut Parser) -> String {
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
    fn write_push_pop(&mut self, command: &mut Parser) -> String {
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
                    "@{0}.{1}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
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
    fn write_init(&mut self) -> String {
        let out = format!("@256\nD=A\n@SP\nM=D\n@SYSRETURN\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@SP\nD=M\n@ARG\n{}M=D\n@SP\nD=M\n@LCL\nM=D\n@SYS.INIT\n0;JMP\n(SYSRETURN)\n", "D=D-1\n".to_string().repeat(5));
        self.function_name.push("Sys.init".to_string());
        out
    }
    fn write_label(&mut self, command: &mut Parser) -> String {
        format!(
            "({0}${1})\n",
            self.function_name.last().unwrap().to_uppercase(),
            command.arg1()
        )
    }
    fn write_goto(&mut self, command: &mut Parser) -> String {
        format!(
            "@{0}${1}\n0;JMP\n",
            self.function_name.last().unwrap().to_uppercase(),
            command.arg1()
        )
    }
    fn write_if(&mut self, command: &mut Parser) -> String {
        format!(
            "@SP\nM=M-1\nD=M\nA=D\nD=M\n@{0}${1}\nD;JNE\n",
            self.function_name.last().unwrap().to_uppercase(),
            command.arg1()
        )
    }
    fn write_call(&mut self, command: &mut Parser) -> String {
        self.func_num += 1;
        format!(
            "@{0}{1}RETURN\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
            command.arg1().to_uppercase(),
            self.func_num,
        ) + "@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
            + "@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
            + "@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
            + "@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
            + &format!(
                "@SP\nD=M\n@ARG\n{}M=D\n",
                "D=D-1\n".to_string().repeat(command.arg2() as usize + 5)
            )
            + "@SP\nD=M\n@LCL\nM=D\n"
            + &format!("@{}\n0;JMP\n", command.arg1().to_uppercase())
            + &format!(
                "({0}{1}RETURN)\n",
                command.arg1().to_uppercase(),
                self.func_num
            )
    }
    fn write_function(&mut self, command: &mut Parser) -> String {
        self.function_name.pop();
        self.function_name.push(command.arg1());
        format!(
            "({0})\n{1}",
            self.function_name.last().unwrap().to_uppercase(),
            "@0\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
                .to_string()
                .repeat(command.arg2() as usize)
        )
    }
    fn write_return(&mut self) -> String {
        let out = "@LCL\nD=M\n@R13\nM=D\n@5\nD=A\n@R13\nA=M-D\nD=M\n@R14\nM=D\n@SP\nM=M-1\nA=M\nD=M\n@ARG\nA=M\nM=D\n@ARG\nD=M+1\n@SP\nM=D\n@R13\nAM=M-1\nD=M\n@THAT\nM=D\n@R13\nAM=M-1\nD=M\n@THIS\nM=D\n@R13\nAM=M-1\nD=M\n@ARG\nM=D\n@R13\nAM=M-1\nD=M\n@LCL\nM=D\n@R14\nA=M\n0;JMP\n";
        out.to_string()
    }
    pub fn write_file(&mut self, dir_name: &str, command: &mut Parser) {
        let outfile = dir_name.to_string() + ".asm";
        let mut write_str: String = String::new();
        write_str += &self.write_init();
        while command.advance() {
            match command.command_type() {
                CommandType::C_ARITHMETIC => write_str += &self.write_arithmetic(command),
                CommandType::C_PUSH | CommandType::C_POP => {
                    write_str += &self.write_push_pop(command)
                }
                CommandType::C_LABEL => write_str += &self.write_label(command),
                CommandType::C_GOTO => write_str += &self.write_goto(command),
                CommandType::C_IF => write_str += &self.write_if(command),
                CommandType::C_CALL => write_str += &self.write_call(command),
                CommandType::C_FUNCTION => write_str += &self.write_function(command),
                CommandType::C_RETURN => write_str += &self.write_return(),
                CommandType::FILE_START => self.file_index += 1,
                _ => (),
            };
        }
        let mut f = BufWriter::new(File::create(outfile).unwrap());
        write!(f, "{}", write_str).unwrap();
    }
}
