use super::parser::*;
use crate::prelude::*;
use std::fs::File;
use std::io::Write;
use std::ops::Add;

/// A public interface for writing VM commands to the output file.
pub trait CodeWriter {
    /// Creates a new instance of the code writer that writes to the specified output file.
    ///
    /// # Arguments
    ///
    /// * `output_file` - The path of the output file to write the VM commands to.
    fn new(output_file: String) -> Self;

    /// Writes an arithmetic operation command to the output file.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the parser that provides information about the command.
    ///
    /// # Errors
    ///
    /// Errors will occur if other.command_type != Arithmetic
    ///
    /// # Examples
    ///
    /// ```no_run
    /// match parser.command_type {
    ///     Some(Command::Arithmetic(_)) => {
    ///         write.write_arithmetic(&parser);
    ///     }
    ///     _ => // your code
    ///}
    /// ```
    fn write_arithmetic(&mut self, other: &ParserClass);

    /// Writes a push or pop command to the output file.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the parser that provides information about the command.
    ///
    /// # Errors
    ///
    /// Errors will occur if other.command_type != PushPop
    ///
    /// # Examples
    ///
    /// ```no_run
    /// match parser.command_type {
    ///     Some(Command::PushPop(_)) => {
    ///         write.write_push_pop(&parser);
    ///     }
    ///     _ => // your code
    ///}
    /// ```
    fn write_push_pop(&mut self, other: &ParserClass);

    /// Writes a branch command to the output file.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the parser that provides information about the command.
    ///
    /// # Errors
    ///
    /// Errors will occur if other.command_type != Branch
    ///
    /// # Examples
    ///
    /// ```no_run
    /// match parser.command_type {
    ///     Some(Command::Branch(_)) => {
    ///         write.write_branch(&parser);
    ///     }
    ///     _ => // your code
    ///}
    /// ```
    fn write_branch(&mut self, other: &ParserClass);

    /// Writes a function command to the output file.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the parser that provides information about the command.
    /// # Errors
    ///
    /// Errors will occur if other.command_type != Function
    ///
    /// # Examples
    ///
    /// ```no_run
    /// match parser.command_type {
    ///     Some(Command::Function(_)) => {
    ///         write.write_function(&parser);
    ///     }
    ///     _ => // your code
    ///}
    /// ```
    fn write_function(&mut self, other: &ParserClass);

    /// Writes the init command to the output file.
    ///
    /// This command is used to initialize when the program begins to start.
    fn write_init(&mut self);
}

/// Represents a code writer responsible for translating VM commands into assembly code and writing them to an output file.
pub struct CodeWriterClass {
    /// File output name
    pub file_name: String,

    /// The output file where the translated assembly code will be written.
    pub file: File,

    /// A mapping of VM arithmetic commands to their corresponding assembly code representations.
    arithmetic_commands: CommandList<String>,

    /// A mapping of VM push/pop commands for internal memory segments that has mapped natively to memory to their corresponding assembly code representations.
    push_pop_internal_commands: CommandList<String>,

    /// A mapping of VM push/pop commands for external memory segments that hasn't mapped natively to memory  to their corresponding assembly code representations.
    push_pop_external_commands: CommandList<String>,

    /// A mapping of VM label branching commands.
    branch_commands: CommandList<String>,

    /// A mapping of VM label function commands.
    function_commands: CommandList<String>,

    state: State,
}

/// CodeWriter is an implementation for the CodeWriterClass, responsible for generating
/// assembly code for the Hack VM language. It provides methods to translate VM commands
/// into Hack assembly code and write the resulting assembly code to an output file.
impl CodeWriter for CodeWriterClass {
    fn new(output_file: String) -> Self {
        // Initialization of various command maps and other internal state.

        #[rustfmt::skip]
        let arithmetic: CommandList<String> = CommandList::new(vec![
            ("add","// add\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M+D\n@SP\nM=M+1",),
            ("sub","// sub\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M-D\n@SP\nM=M+1",),
            ("neg", "// neg\n@SP\nM=M-1\nA=M\nD=M\nM=M-D\nM=M-D\n@SP\nM=M+1"),
            ("eq", "// eq\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@CON_TRUE_{i}\nD;JEQ\n@SP\nA=M\nM=0\n@CON_FINISH_{i}\n0;JMP\n(CON_TRUE_{i})\n@SP\nA=M\nM=-1\n(CON_FINISH_{i})\n@SP\nM=M+1"),
            ("gt", "// gt\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@CON_TRUE_{i}\nD;JGT\n@SP\nA=M\nM=0\n@CON_FINISH_{i}\n0;JMP\n(CON_TRUE_{i})\n@SP\nA=M\nM=-1\n(CON_FINISH_{i})\n@SP\nM=M+1"),
            ("lt", "// lt\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@CON_TRUE_{i}\nD;JLT\n@SP\nA=M\nM=0\n@CON_FINISH_{i}\n0;JMP\n(CON_TRUE_{i})\n@SP\nA=M\nM=-1\n(CON_FINISH_{i})\n@SP\nM=M+1"),
            ("and", "// and\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M&D\n@SP\nM=M+1"),
            ("or", "// or\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M|D\n@SP\nM=M+1"),
            ("not", "// not\n@SP\nM=M-1\nA=M\nM=!M\n@SP\nM=M+1\n")
            ]);

        #[rustfmt::skip]
        let push_pop_internal :CommandList<String> = CommandList::new(vec![
            ("push", "// push {segment} {i} \n@{i}\nD=A\n@{segment}\nM=M+D\nA=M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@{i}\nD=A\n@{segment}\nM=M-D"),
            ("pop", "// pop {segment} {i} \n@{i}\nD=A\n@{segment}\nM=M+D\n@SP\nM=M-1\nA=M\nD=M\n@{segment}\nA=M\nM=D\n@{i}\nD=A\n@{segment}\nM=M-D")
        ]);

        #[rustfmt::skip]
        let push_pop_ekstenal: CommandList<String> = CommandList::new(vec![
            ("push_constant", "// push constant {i}\n@{i}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
            ("push_static", "// push static {i}\n@{file_name}.{i}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
            ("pop_static", "// pop static {i}\n@SP\nM=M-1\nA=M\nD=M\n@{file_name}.{i}\nM=D",),
            ("pop_temp", "// pop temp {i}\n@SP\nM=M-1\nA=M\nD=M\n@{temp}\nM=D",),
            ("push_temp", "// push temp {i}\n@{temp}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
            ("pop_pointer", "// pop pointer {i}\n@SP\nM=M-1\nA=M\nD=M\n@{segment}\nM=D",),
            ("push_pointer", "// push pointer {i}\n@{segment}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
        ]);

        #[rustfmt::skip]
        let branch: CommandList<String> = CommandList::new(vec![
            ("label", "// label \n({label_name})",),
            ("goto", "// goto \n@{label_name}\n0;JMP",),
            ("if-goto", "// if-goto \n@SP\nM=M-1\nA=M\nD=M\n@{label_name}\nD;JNE",),
        ]);

        #[rustfmt::skip]
        let function:CommandList<String> = CommandList::new(vec![
            ("function", "// function {function_name} {Vars}\n({function_name})",),
            ("call", "// call {function_name} {Args}\n@{function_name}.ret.{i}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\nD=M\n@5\nD=D-A\n@{Args}\nD=D-A\n@ARG\nM=D\n@SP\nD=M\n@LCL\nM=D\n@{function_name}\n0;JMP\n({function_name}.ret.{i})",),
            ("return", "// return\n@LCL\nD=M\n@13\nM=D\nD=M\n@5\nD=D-A\nA=D\nD=M\n@14\nM=D\n@SP\nM=M-1\nA=M\nD=M\n@ARG\nA=M\nM=D\nD=A\n@SP\nM=D\nM=M+1\n@13\nD=M-1\nA=D\nD=M\n@THAT\nM=D\n@13\nA=M\nD=A\n@2\nD=D-A\nA=D\nD=M\n@THIS\nM=D\n@13\nA=M\nD=A\n@3\nD=D-A\nA=D\nD=M\n@ARG\nM=D\n@13\nA=M\nD=A\n@4\nD=D-A\nA=D\nD=M\n@LCL\nM=D\n@14\nA=M\n0;JMP",),
        ]);

        // Create a new instance of CodeWriterClass with the initialized data.
        CodeWriterClass {
            file_name: output_file.to_string(),
            file: File::create(output_file).unwrap(),
            arithmetic_commands: arithmetic,
            push_pop_internal_commands: push_pop_internal,
            push_pop_external_commands: push_pop_ekstenal,
            branch_commands: branch,
            function_commands: function,
            state: State::default(),
        }
    }

    fn write_arithmetic(&mut self, other: &ParserClass) {
        // List of supported arithmetic commands that require an additional integer argument
        let if_condition: List<String> =
            List::new(vec!["gt".to_string(), "lt".to_string(), "eq".to_string()]);

        // Check if `other` contains an arithmetic command
        if let Some(Command::Arithmetic(command)) = &other.command_type {
            // Retrieve the corresponding assembly code for the arithmetic command
            let mut to_write = self.arithmetic_commands.get(command).to_string();

            // If the command requires an additional integer argument, replace "{i}" in the assembly code with a unique identifier
            if if_condition.is_exist(command) {
                to_write = to_write.replace("{i}", &self.state.get_logical());
                self.state.inc_logical(); // Increment the unique identifier for the next command
            }

            // Write the resulting assembly code to the output file
            writeln!(self.file, "{to_write}").unwrap();
        } else {
            // Panic if `other` does not contain an arithmetic command
            panic!(
                "Command {:?} is not an arithmetic command",
                other.command_type
            );
        }
    }

    fn write_push_pop(&mut self, other: &ParserClass) {
        // Check if the command is of type PushPop.
        let Some(Command::PushPop(command)) = &other.command_type else {
            // If the command type is not recognized, panic with an error message.
            panic!("Command {:?} is not a valid command.", &other.command_type);
        };
        // Match the segment type (external or internal).
        match &other.segment_type {
            // For external segments (static, temp, pointer), generate the assembly code.
            Some(Segment::External(_segment)) => {
                // Create a key to look up the corresponding assembly code in the map.
                let key = command.clone().add("_").add(_segment);

                // Get the corresponding assembly code for the push or pop command.
                let mut to_write = self.push_pop_external_commands.get(&key).to_string();

                // Initialize variables to be used for segment-specific processing.
                let (mut segment_to_add, mut temp_address) = (String::new(), 5);

                // Process the specific segments (static, temp, pointer).
                if _segment == "static" {
                    segment_to_add = self.file_name.to_string();
                } else if _segment == "temp" {
                    segment_to_add = "Temp".to_string();
                    temp_address += other.index.unwrap();
                } else if _segment == "pointer" {
                    if other.index.unwrap() == 1 {
                        segment_to_add = "THAT".to_string();
                    } else {
                        segment_to_add = "THIS".to_string();
                    } 
                }

                // Replace placeholders in the assembly code with the appropriate values.
                to_write = to_write
                    .replace("{i}", &other.index.unwrap().clone().to_string())
                    .replace("{segment}", &segment_to_add)
                    .replace("{temp}", &temp_address.to_string())
                    .replace("{file_name}", &self.file_name[..self.file_name.len() - 3]);

                // Write the translated assembly code to the output file.
                writeln!(self.file, "{to_write}").unwrap();
            }

            // For internal segments (local, argument, this, that), generate the assembly code.
            Some(Segment::Internal(segment)) => {
                // Get the corresponding assembly code for the push or pop command.
                let mut to_write = self
                    .push_pop_internal_commands
                    .get(&command.to_string())
                    .to_string();

                // Initialize a variable to be used for segment-specific processing.
                let mut segment_to_add: String = String::new();

                // Process the specific segments (local, argument, this, that).
                if segment == "local" {
                    segment_to_add = "LCL".to_string();
                } else if segment == "argument" {
                    segment_to_add = "ARG".to_string();
                } else if segment == "this" {
                    segment_to_add = "THIS".to_string();
                } else if segment == "that" {
                    segment_to_add = "THAT".to_string();
                }

                // Replace placeholders in the assembly code with the appropriate values.
                to_write = to_write
                    .replace("{i}", &other.index.unwrap().clone().to_string())
                    .replace("{segment}", &segment_to_add);

                // Write the translated assembly code to the output file.
                writeln!(self.file, "{to_write}").unwrap();
            }

            // If the segment type is not recognized, panic with an error message.
            _ => panic!("Segment {:?} is not a valid segment.", &other.segment_type),
        }
    }

    fn write_branch(&mut self, other: &ParserClass) {
        // Check if the command type is a branch command (label, goto, if-goto).
        let Some(Command::Branch(command)) = &other.command_type else {
            // If the command type is not a branch command, panic with an error message.
            panic!(
                "Command {:?} is not a valid branch command",
                other.command_type
            );
        };

        // Split the current command into parts, assuming it is space-separated.
        let label: Vec<&str> = other.current_command.split(' ').collect();

        // Get the corresponding assembly code for the branch command from the map.
        let mut to_write = self.branch_commands.get(command).to_string();

        // Replace the placeholder "{label_name}" in the assembly code with the actual label name.
        to_write = to_write.replace("{label_name}", label[1]);

        // Write the translated assembly code to the output file.
        writeln!(self.file, "{to_write}").unwrap();
    }

    fn write_function(&mut self, other: &ParserClass) {
        // Check if the command type is a function command.
        let Some(Command::Function(command)) = &other.command_type else {
            // If the command type is not recognized as a function command, panic with an error message.
            panic!(
                "Command {:?} is not a valid function command",
                other.command_type
            );
        };

        // Split the current command into parts, assuming it is space-separated.
        let _command: Vec<&str> = other.current_command.split(' ').collect();

        // Get the corresponding assembly code for the function command from the map.
        let mut to_write = self.function_commands.get(command).to_string();

        // Process the command if it contains additional arguments (function name and argument count).
        if _command.len() > 1 {
            // Replace placeholders in the assembly code with the actual function and file names, Args, Vars, and count.
            to_write = to_write
                .replace("{function_name}", _command[1])
                .replace("{file_name}", &self.file_name[..self.file_name.len() - 4])
                .replace("{Args}", _command[2])
                .replace("{Vars}", _command[2])
                .replace("{i}", &self.state.get_function());

            // Increment the function count for subsequent function declarations.
            self.state.inc_function();
        }

        // Write the translated assembly code to the output file.
        writeln!(self.file, "{to_write}").unwrap();

        // If the command is a "function" command, add local variables to the function's stack frame.
        if _command[0] == "function" {
            let vars: usize = _command[2].parse::<usize>().unwrap();
            // Iterate over the number of local variables and initialize them to 0 on the stack frame.
            for i in 0..vars {
                writeln!(
                    self.file,
                    "// Add local var(s)\n@{i}\nD=A\n@LCL\nA=M+D\nM=0\n@SP\nM=M+1"
                )
                .unwrap();
            }
        }
    }

    fn write_init(&mut self) {
        // Write the bootstrap code to the output file.
        writeln!(self.file, "// Bootstrap code\n@256\nD=A\n@SP\nM=D\n@returnAddress.0\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\nD=M\n@5\nD=D-A\n@0\nD=D-A\n@ARG\nM=D\n@SP\nD=M\n@LCL\nM=D\n@Sys.init\n0;JMP\n(returnAddress.0)\n(while)\n@while\n0;JMP").unwrap();
    }
}
