pub mod modules {
    /// Represents different types of commands that can be parsed from the input file.
    #[derive(Debug)]
    pub enum Command {
        /// Represents an arithmetic operation command. ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"]
        Arithmetic(String),
        /// Represents a PushPop command.
        PushPop(String),
        /// Represents a branch command.
        Branch(String),
        /// Represents a Function command.
        Function(String),
    }
    impl PartialEq for Command {
        /// Compares two `Command` instances and returns true if they are equal in type.
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Command::PushPop(_), Command::PushPop(_)) => true,
                (Command::Arithmetic(_), Command::Arithmetic(_)) => true,
                (Command::Branch(_), Command::Branch(_)) => true,
                _ => false,
            }
        }
    }

    /// Represents different types of memory segments that can be parsed from the input file.
    #[derive(Clone, Debug)]
    pub enum Segment {
        /// Represents an internal memory segment. ["local", "argument", "this", "that"]
        Internal(String),
        /// Represents an external memory segment. ["constant", "static", "temp", "pointer"]
        External(String),
    }
    impl PartialEq for Segment {
        /// Compares two `Segment` instances and returns true if they are equal in type.
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Segment::Internal(_), Segment::Internal(_)) => true,
                (Segment::External(_), Segment::External(_)) => true,
                _ => false,
            }
        }
    }
}
pub mod parser {
    use super::modules;
    use modules::{Command, Segment};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    /// A public interface for parsing the input file and extracting commands.
    pub trait ParserPublic {
        /// Creates a new instance of the parser.
        ///
        /// # Arguments
        ///
        /// * `input_file` - A `BufReader<File>` that reads the input file.
        fn new(input_file: BufReader<File>) -> Self;

        /// Checks if there are more commands to be parsed from the input file.
        ///
        /// # Returns
        ///
        /// `true` if there are more commands, `false` otherwise.
        fn has_more_commands(&mut self) -> bool;
    }

    /// A private interface for parsing the input file and extracting commands and segments.
    trait ParserPrivate {
        /// Advances to the next command in the input file.
        fn advance(&mut self);

        /// Retrieves the type of the current command.
        ///
        /// # Returns
        ///
        /// A `Command` representing the type of the current command.
        fn command_type(&mut self) -> Option<Command>;

        /// Retrieves the type of the current memory segment.
        ///
        /// # Returns
        ///
        /// A `Segment` representing the type of the current memory segment.
        fn segment_type(&self) -> Option<Segment>;
    }

    /// Represents a parser responsible for reading VM commands from an input file and extracting relevant information.
    pub struct ParserClass {
        /// The input file buffer that reads the VM commands.
        file: BufReader<File>,

        /// The current VM command being processed.
        pub current_command: String,

        /// The next VM command instruction to be processed.
        next_instruction: String,

        /// A vector containing supported VM arithmetic commands for parsing.
        arithmetic_commands: Vec<String>,

        /// A vector containing supported VM push and pop commands for parsing.
        push_pop_commands: Vec<String>,

        /// A vector containing supported VM branch commands for parsing.
        branch_commands: Vec<String>,

        /// A vector containing supported VM function commands for parsing.
        function_commands: Vec<String>,

        /// The type of the current VM command.
        pub command_type: Option<Command>,

        /// The type of the memory segment associated with the current VM command (if applicable).
        pub segment_type: Option<Segment>,

        /// The index or offset used in VM commands that require it (e.g., push/pop operations).
        pub index: Option<i32>,
    }

    impl ParserPublic for ParserClass {
        fn new(input_file: BufReader<File>) -> Self {
            let aritmetic: Vec<String> =
                vec!["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"]
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect();
            let push_pop: Vec<String> = vec!["push", "pop"]
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            let branch: Vec<String> = vec!["label", "if-goto", "goto"]
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            let function: Vec<String> = vec!["function", "call", "return"]
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            ParserClass {
                file: input_file,
                current_command: String::new(),
                next_instruction: String::new(),
                arithmetic_commands: aritmetic,
                push_pop_commands: push_pop,
                branch_commands: branch,
                function_commands: function,
                command_type: None,
                segment_type: None,
                index: None,
            }
        }

        fn has_more_commands(&mut self) -> bool {
            self.next_instruction.clear();
            loop {
                if let Ok(bytes) = self.file.read_line(&mut self.next_instruction) {
                    if bytes > 0 {
                        let to_verified: Vec<String> = self
                            .next_instruction
                            .clone()
                            .split('/')
                            .map(|x| x.to_string())
                            .collect();
                        self.next_instruction = to_verified[0].clone().trim().to_string();
                        if self.next_instruction.chars().next().unwrap_or('/') == '/' {
                            continue;
                        }
                        self.advance();
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
    }

    impl ParserPrivate for ParserClass {
        fn advance(&mut self) {
            self.current_command = self.next_instruction.clone();
            self.command_type = self.command_type();
            if let Some(Command::PushPop(_)) = self.command_type {
                self.segment_type = self.segment_type();
            }
        }

        fn command_type(&mut self) -> Option<Command> {
            let a: Vec<&str> = self.current_command.split(' ').collect();
            if !self.current_command.is_empty() {
                match a[0].to_lowercase().trim() {
                    command if self.push_pop_commands.contains(&command.to_string()) => {
                        self.index = Some(a[2].trim().parse::<i32>().unwrap());
                        return Some(Command::PushPop(command.to_string()));
                    }
                    command if self.arithmetic_commands.contains(&command.to_string()) => {
                        self.index = None;
                        return Some(Command::Arithmetic(command.to_string()));
                    }
                    command if self.branch_commands.contains(&command.to_string()) => {
                        self.index = None;
                        return Some(Command::Branch(command.to_string()));
                    }
                    command if self.function_commands.contains(&command.to_string()) => {
                        self.index = None;
                        return Some(Command::Function(command.to_string()));
                    }
                    _ => {
                        self.index = None;
                        return None;
                    }
                };
            }
            None
        }

        fn segment_type(&self) -> Option<Segment> {
            let a: Vec<&str> = self.current_command.split(' ').collect();
            let internal: Vec<String> = vec!["local", "argument", "this", "that"]
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            let external: Vec<String> = vec!["constant", "static", "temp", "pointer"]
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            match a[1].to_lowercase().trim() {
                a if internal.contains(&a.to_string()) => {
                    return Some(Segment::Internal(a.to_string()));
                }
                a if external.contains(&a.to_string()) => {
                    return Some(Segment::External(a.to_string()));
                }
                _ => return None,
            };
        }
    }
}

pub mod code_writer {
    use super::modules;
    use super::parser;
    use modules::{Command, Segment};
    use parser::ParserClass;
    use std::collections::HashMap;
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
        fn write_arithmetic(&mut self, other: &ParserClass);

        /// Writes a push or pop command to the output file.
        ///
        /// # Arguments
        ///
        /// * `other` - A reference to the parser that provides information about the command.
        fn write_push_pop(&mut self, other: &ParserClass);

        fn write_branch(&mut self, other: &ParserClass);

        fn write_function(&mut self, other: &ParserClass);

        fn write_init(&mut self);
    }

    /// Represents a code writer responsible for translating VM commands into assembly code and writing them to an output file.
    pub struct CodeWriterClass {
        /// File output name
        pub file_name: String,

        /// The output file where the translated assembly code will be written.
        pub file: File,

        /// A mapping of VM arithmetic commands to their corresponding assembly code representations.
        arithmetic_commands: HashMap<String, String>,

        /// A mapping of VM push/pop commands for internal memory segments that has mapped natively to memory to their corresponding assembly code representations.
        push_pop_internal_commands: HashMap<String, String>,

        /// A mapping of VM push/pop commands for external memory segments that hasn't mapped natively to memory  to their corresponding assembly code representations.
        push_pop_external_commands: HashMap<String, String>,

        /// A mapping of VM label branching commands.
        branch_commands: HashMap<String, String>,

        /// A mapping of VM label function commands.
        function_commands: HashMap<String, String>,

        /// A counter used to generate unique labels for conditional jumps (used in logic commands).
        logical_count: i32,

        /// A counter used to generate unique labels for function jumps (used in logic commands).
        function_count: i32,
    }

    impl CodeWriter for CodeWriterClass {
        fn new(output_file: String) -> Self {
            #[rustfmt::skip]
            let arithmetic: HashMap<String, String> = vec![
                ("add","// add\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M+D\n@SP\nM=M+1",),
                ("sub","// sub\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M-D\n@SP\nM=M+1",),
                ("neg", "// neg\n@SP\nM=M-1\nA=M\nD=M\nM=M-D\nM=M-D\n@SP\nM=M+1"),
                ("eq", "// eq\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@CON_TRUE_{i}\nD;JEQ\n@SP\nA=M\nM=0\n@CON_FINISH_{i}\n0;JMP\n(CON_TRUE_{i})\n@SP\nA=M\nM=-1\n(CON_FINISH_{i})\n@SP\nM=M+1"),
                ("gt", "// gt\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@CON_TRUE_{i}\nD;JGT\n@SP\nA=M\nM=0\n@CON_FINISH_{i}\n0;JMP\n(CON_TRUE_{i})\n@SP\nA=M\nM=-1\n(CON_FINISH_{i})\n@SP\nM=M+1"),
                ("lt", "// lt\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@CON_TRUE_{i}\nD;JLT\n@SP\nA=M\nM=0\n@CON_FINISH_{i}\n0;JMP\n(CON_TRUE_{i})\n@SP\nA=M\nM=-1\n(CON_FINISH_{i})\n@SP\nM=M+1"),
                ("and", "// and\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M&D\n@SP\nM=M+1"),
                ("or", "// or\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M|D\n@SP\nM=M+1"),
                ("not", "// not\n@SP\nM=M-1\nA=M\nM=!M\n@SP\nM=M+1\n")
            ].into_iter().map(|(x,y)| (x.to_string(),y.to_string())).collect();

            #[rustfmt::skip]
            let push_pop_internal :HashMap<String, String> = vec![
                ("push", "// push {segment} {i} \n@{i}\nD=A\n@{segment}\nM=M+D\nA=M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@{i}\nD=A\n@{segment}\nM=M-D"),
                ("pop", "// pop {segment} {i} \n@{i}\nD=A\n@{segment}\nM=M+D\n@SP\nM=M-1\nA=M\nD=M\n@{segment}\nA=M\nM=D\n@{i}\nD=A\n@{segment}\nM=M-D")
            ].into_iter().map(|(x,y)| (x.to_string(),y.to_string())).collect();

            #[rustfmt::skip]
            let push_pop_ekstenal: HashMap<String, String> = vec![
                ("push_constant", "// push constant {i}\n@{i}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
                ("push_static", "// push static {i}\n@{file_name}.{i}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
                ("pop_static", "// pop static {i}\n@SP\nM=M-1\nA=M\nD=M\n@{file_name}.{i}\nM=D",),
                ("pop_temp", "// pop temp {i}\n@SP\nM=M-1\nA=M\nD=M\n@{temp}\nM=D",),
                ("push_temp", "// push temp {i}\n@{temp}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
                ("pop_pointer", "// pop pointer {i}\n@SP\nM=M-1\nA=M\nD=M\n@{segment}\nM=D",),
                ("push_pointer", "// push pointer {i}\n@{segment}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
            ]
            .into_iter()
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect();

            #[rustfmt::skip]
            let branch: HashMap<String, String> = vec![
                ("label", "// label \n({label_name})",),
                ("goto", "// goto \n@{label_name}\n0;JMP",),
                ("if-goto", "// if-goto \n@SP\nM=M-1\nA=M\nD=M\n@{label_name}\nD;JNE",),
            ]
            .into_iter()
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect();

            #[rustfmt::skip]
            let function: HashMap<String, String> = vec![
                ("function", "// function {function_name} {Vars}\n({function_name})",),
                ("call", "// call {function_name} {Args}\n@{function_name}.ret.{i}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\nD=M\n@5\nD=D-A\n@{Args}\nD=D-A\n@ARG\nM=D\n@SP\nD=M\n@LCL\nM=D\n@{function_name}\n0;JMP\n({function_name}.ret.{i})",),
                ("return", "// return\n@LCL\nD=M\n@13\nM=D\nD=M\n@5\nD=D-A\nA=D\nD=M\n@14\nM=D\n@SP\nM=M-1\nA=M\nD=M\n@ARG\nA=M\nM=D\nD=A\n@SP\nM=D\nM=M+1\n@13\nD=M-1\nA=D\nD=M\n@THAT\nM=D\n@13\nA=M\nD=A\n@2\nD=D-A\nA=D\nD=M\n@THIS\nM=D\n@13\nA=M\nD=A\n@3\nD=D-A\nA=D\nD=M\n@ARG\nM=D\n@13\nA=M\nD=A\n@4\nD=D-A\nA=D\nD=M\n@LCL\nM=D\n@14\nA=M\n0;JMP",),
            ]
            .into_iter()
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect();

            CodeWriterClass {
                file_name: output_file.to_string(),
                file: File::create(output_file.to_string()).unwrap(),
                arithmetic_commands: arithmetic,
                push_pop_internal_commands: push_pop_internal,
                push_pop_external_commands: push_pop_ekstenal,
                branch_commands: branch,
                function_commands: function,
                logical_count: 0,
                function_count: 1,
            }
        }

        fn write_arithmetic(&mut self, other: &ParserClass) {
            let if_condition: Vec<String> =
                vec!["gt".to_string(), "lt".to_string(), "eq".to_string()];
            if let Some(Command::Arithmetic(command)) = &other.command_type {
                let mut to_write = self.arithmetic_commands.get(command).unwrap().to_string();
                if if_condition.contains(&command) {
                    to_write = to_write.replace("{i}", &self.logical_count.clone().to_string());
                    self.logical_count += 1;
                };
                writeln!(self.file, "{}", to_write).unwrap();
            } else {
                panic!("Command {:?} is not arithmetic command", other.command_type);
            }
        }

        fn write_push_pop(&mut self, other: &ParserClass) {
            if let Some(Command::PushPop(command)) = &other.command_type {
                match &other.segment_type {
                    Some(Segment::External(segment)) => {
                        let key = command.clone().add(&"_").add(&segment);
                        let mut to_write = self
                            .push_pop_external_commands
                            .get(&key)
                            .unwrap()
                            .to_string();
                        let (mut segment_to_add, mut temp_address) = (String::new(), 5);
                        if segment == "static" {
                            segment_to_add = self.file_name.to_string()
                        } else if segment == "temp" {
                            temp_address += other.index.unwrap();
                            segment_to_add = "Temp".to_string();
                        } else if segment == "pointer" {
                            segment_to_add = "THIS".to_string();
                            if other.index.unwrap() == 1 {
                                segment_to_add = "THAT".to_string();
                            }
                        }
                        to_write = to_write
                            .replace("{i}", &other.index.unwrap().clone().to_string())
                            .replace("{segment}", &segment_to_add)
                            .replace("{temp}", &temp_address.to_string())
                            .replace("{file_name}", &self.file_name[..self.file_name.len() - 4]);
                        writeln!(self.file, "{}", to_write).unwrap();
                    }
                    Some(Segment::Internal(segment)) => {
                        let mut to_write = self
                            .push_pop_internal_commands
                            .get(&command.to_string())
                            .unwrap()
                            .to_string();
                        let mut segment_too_add: String = String::new();
                        if segment == "local" {
                            segment_too_add = "LCL".to_string();
                        } else if segment == "argument" {
                            segment_too_add = "ARG".to_string()
                        } else if segment == "this" {
                            segment_too_add = "THIS".to_string()
                        } else if segment == "that" {
                            segment_too_add = "THAT".to_string()
                        }
                        to_write = to_write
                            .replace("{i}", &other.index.unwrap().clone().to_string())
                            .replace("{segment}", &segment_too_add);
                        writeln!(self.file, "{}", to_write).unwrap();
                    }
                    _ => panic!("Segment {:?} is not segment.", &other.segment_type),
                }
            } else {
                panic!("Command {:?} is not command function.", &other.command_type);
            };
        }

        fn write_branch(&mut self, other: &ParserClass) {
            if let Some(Command::Branch(command)) = &other.command_type {
                let a: Vec<&str> = other.current_command.split(' ').collect();
                let mut to_write = self.branch_commands.get(command).unwrap().to_string();
                to_write = to_write.replace("{label_name}", a[1]);
                writeln!(self.file, "{}", to_write).unwrap();
            } else {
                panic!("Command {:?} is not branch command", other.command_type);
            }
        }

        fn write_function(&mut self, other: &ParserClass) {
            if let Some(Command::Function(command)) = &other.command_type {
                let a: Vec<&str> = other.current_command.split(' ').collect();
                let mut to_write = self.function_commands.get(command).unwrap().to_string();
                if a.len() > 1 {
                    to_write = to_write
                        .replace("{function_name}", a[1])
                        .replace("{file_name}", &self.file_name[..self.file_name.len() - 4])
                        .replace("{Args}", a[2])
                        .replace("{Vars}", a[2])
                        .replace("{i}", &self.function_count.to_string());
                    self.function_count += 1;
                }
                writeln!(self.file, "{}", to_write).unwrap();
                if a[0] == "function" {
                    let vars: usize = a[2].clone().parse::<usize>().unwrap();
                    dbg!("{}", &a);
                    for i in 0..vars {
                        writeln!(
                            self.file,
                            "// Add local var(s)\n@{}\nD=A\n@LCL\nA=M+D\nM=0\n@SP\nM=M+1",
                            i
                        )
                        .unwrap();
                    }
                }
            } else {
                panic!("Command {:?} is not function command", other.command_type);
            }
        }

        fn write_init(&mut self) {
            writeln!(self.file, "// Bootstrap code\n@256\nD=A\n@SP\nM=D\n@returnAddress.0\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\nD=M\n@5\nD=D-A\n@0\nD=D-A\n@ARG\nM=D\n@SP\nD=M\n@LCL\nM=D\n@Sys.init\n0;JMP\n(returnAddress.0)\n(while)\n@while\n0;JMP").unwrap();
        }
    }
}
