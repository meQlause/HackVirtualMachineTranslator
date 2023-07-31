// push constant 10
@10
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop LCL 0 local/arg/this/that
@0
D=A
@LCL
M=M+D
@SP
M=M-1
A=M
D=M
@LCL
A=M
M=D
@0
D=A
@LCL
M=M-D

// push constant 21
@21
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 22
@22
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop ARG 2 local/arg/this/that
@2
D=A
@ARG
M=M+D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@2
D=A
@ARG
M=M-D

// pop ARG 1 local/arg/this/that
@1
D=A
@ARG
M=M+D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@1
D=A
@ARG
M=M-D

// push constant 36
@36
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop THIS 6 local/arg/this/that
@6
D=A
@THIS
M=M+D
@SP
M=M-1
A=M
D=M
@THIS
A=M
M=D
@6
D=A
@THIS
M=M-D

// push constant 42
@42
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 45
@45
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop THAT 5 local/arg/this/that
@5
D=A
@THAT
M=M+D
@SP
M=M-1
A=M
D=M
@THAT
A=M
M=D
@5
D=A
@THAT
M=M-D

// pop THAT 2 local/arg/this/that
@2
D=A
@THAT
M=M+D
@SP
M=M-1
A=M
D=M
@THAT
A=M
M=D
@2
D=A
@THAT
M=M-D

// push constant 510
@510
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop temp 6
@SP
M=M-1
A=M
D=M
@Temp6
M=D

// push LCL {int} local/arg/this/that
@0
D=A
@LCL
M=M+D
D=M
@SP
A=M
M=D
M=M+1
@0
D=A
@LCL
M=M-D

// push THAT {int} local/arg/this/that
@5
D=A
@THAT
M=M+D
D=M
@SP
A=M
M=D
M=M+1
@5
D=A
@THAT
M=M-D

// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1

// push ARG {int} local/arg/this/that
@1
D=A
@ARG
M=M+D
D=M
@SP
A=M
M=D
M=M+1
@1
D=A
@ARG
M=M-D

// sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1

// push THIS {int} local/arg/this/that
@6
D=A
@THIS
M=M+D
D=M
@SP
A=M
M=D
M=M+1
@6
D=A
@THIS
M=M-D

// push THIS {int} local/arg/this/that
@6
D=A
@THIS
M=M+D
D=M
@SP
A=M
M=D
M=M+1
@6
D=A
@THIS
M=M-D

// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1

// sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1

// push temp 6
@Temp6
D=M
@SP
A=M
M=D
M=M+1

// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1

