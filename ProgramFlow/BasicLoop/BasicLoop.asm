@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@LCL
M=M+D
@0
@SP
M=M-1
D=M
A=D
D=M
@LCL
A=M
M=D
@0
D=A
@LCL
M=M-D
(LOOP_START)
@0
D=A
@ARG
A=M
A=A+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@LCL
A=M
A=A+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
D=M
A=D
D=M
A=A-1
M=M+D
@0
D=A
@LCL
M=M+D
@0
@SP
M=M-1
D=M
A=D
D=M
@LCL
A=M
M=D
@0
D=A
@LCL
M=M-D
@0
D=A
@ARG
A=M
A=A+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
D=M
A=D
D=M
A=A-1
M=M-D
@0
D=A
@ARG
M=M+D
@0
@SP
M=M-1
D=M
A=D
D=M
@ARG
A=M
M=D
@0
D=A
@ARG
M=M-D
@0
D=A
@ARG
A=M
A=A+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
D=M
A=D
D=M
@LOOP_START
D;JNE
@0
D=A
@LCL
A=M
A=A+D
D=M
@SP
A=M
M=D
@SP
M=M+1
