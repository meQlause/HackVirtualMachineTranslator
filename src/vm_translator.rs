pub mod modules {
    /// Represents different types of commands that can be parsed from the input file.
    #[derive(Debug)]
    pub enum Command {
        /// Represents a command that does not fall into any other category.
        None,
        /// Represents an arithmetic operation command. ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"]
        Arithmetic(String),
        /// Represents a PushPop command.
        PushPop(String),
        /// Represents a branch command.
        Branch(String),
    }
    impl PartialEq for Command {
        /// Compares two `Command` instances and returns true if they are equal in type.
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Command::PushPop(_), Command::PushPop(_)) => true,
                (Command::Arithmetic(_), Command::Arithmetic(_)) => true,
                (Command::Branch(_), Command::Branch(_)) => true,
                (Command::None, Command::None) => true,
                _ => false,
            }
        }
    }

    /// Represents different types of memory segments that can be parsed from the input file.
    #[derive(Clone, Debug)]
    pub enum Segment {
        /// Represents a command that does not fall into any other category.
        None,
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
                (Segment::None, Segment::None) => true,
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
        fn command_type(&mut self) -> Command;

        /// Retrieves the type of the current memory segment.
        ///
        /// # Returns
        ///
        /// A `Segment` representing the type of the current memory segment.
        fn segment_type(&self) -> Segment;
    }

    /// Represents a parser responsible for reading VM commands from an input file and extracting relevant information.
    pub struct ParserClass {
        /// The input file buffer that reads the VM commands.
        file: BufReader<File>,

        /// The current VM command being processed.
        current_command: String,

        /// The next VM command instruction to be processed.
        next_instruction: String,

        /// A vector containing supported VM arithmetic commands for parsing.
        arithmetic_commands: Vec<String>,

        /// A vector containing supported VM push and pop commands for parsing.
        push_pop_commands: Vec<String>,

        /// A vector containing supported VM branch pop commands for parsing.
        branch_commands: Vec<String>,

        /// The type of the current VM command.
        pub command_type: Command,

        /// The type of the memory segment associated with the current VM command (if applicable).
        pub segment_type: Segment,

        /// The index or offset used in VM commands that require it (e.g., push/pop operations).
        pub index: i32,
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
            let branch: Vec<String> = vec!["label", "if_goto", "goto"]
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
                command_type: Command::None,
                segment_type: Segment::None,
                index: -1,
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
            self.segment_type = self.segment_type()
        }

        fn command_type(&mut self) -> Command {
            let a: Vec<&str> = self.current_command.split(' ').collect();
            if !self.current_command.is_empty() {
                match a[0].to_lowercase().trim() {
                    b if self.push_pop_commands.contains(&b.to_string()) => {
                        self.index = a[2].trim().parse::<i32>().unwrap();
                        return Command::PushPop(b.to_string());
                    }
                    b if self.arithmetic_commands.contains(&b.to_string()) => {
                        self.index = -1;
                        return Command::Arithmetic(b.to_string());
                    }
                    _ => {
                        self.index = -1;
                        return Command::None;
                    }
                };
            }
            Command::None
        }

        fn segment_type(&self) -> Segment {
            let a: Vec<&str> = self.current_command.split(' ').collect();
            let internal: Vec<String> = vec!["local", "argument", "this", "that"]
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            let external: Vec<String> = vec!["constant", "static", "temp", "pointer"]
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            if !self.current_command.is_empty() && a.len() > 1 {
                match a[1].to_lowercase().trim() {
                    a if internal.contains(&a.to_string()) => {
                        return Segment::Internal(a.to_string());
                    }
                    a if external.contains(&a.to_string()) => {
                        return Segment::External(a.to_string());
                    }
                    _ => return Segment::None,
                };
            }
            Segment::None
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

        /// A counter used to generate unique labels for conditional jumps (used in logic commands).
        if_count: i32,

        /// A counter used to generate unique labels for loop jumps (used in branching commands).
        loop_count: i32,
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
                ("push_static", "// push static {i}\n@aritmetic.{i}\n@{static}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
                ("pop_static", "// pop static {i}\n@SP\nM=M-1\nA=M\nD=M\n@aritmetic.{i}\n@{static}\nM=D",),
                ("pop_temp", "// pop temp {i}\n@SP\nM=M-1\nA=M\nD=M\n@{temp}\nM=D",),
                ("push_temp", "// push temp {i}\n@{temp}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
                ("pop_pointer", "// pop pointer {i}\n@SP\nM=M-1\nA=M\nD=M\n@{segment}\nM=D",),
                ("push_pointer", "// push pointer {i}\n@{segment}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",),
            ]
            .into_iter()
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect();

            #[rustfmt::skip]
            let label: HashMap<String, String> = vec![
                ("label", "// label \n({label_name}{i})",),
                ("goto", "// goto \n@{label_name}{i}\n0;JMP",),
                ("if-goto", "// if-goto \n@SP\nM=M-1\nA=M\nD=M\n@{label_name}{i}\n@D;JGT",),
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
                if_count: 0,
                loop_count: 0,
            }
        }

        fn write_arithmetic(&mut self, other: &ParserClass) {
            let if_condition: Vec<String> =
                vec!["gt".to_string(), "lt".to_string(), "eq".to_string()];
            if let Command::Arithmetic(x) = &other.command_type {
                let mut to_write = self.arithmetic_commands.get(x).unwrap().to_string();
                if if_condition.contains(&x) {
                    to_write = to_write.replace("{i}", &self.if_count.clone().to_string());
                    self.if_count += 1;
                };
                writeln!(self.file, "{}", to_write).unwrap();
            } else {
                panic!("Command {:?} is not arithmetic command", other.command_type);
            }
        }

        fn write_push_pop(&mut self, other: &ParserClass) {
            if let Command::PushPop(command) = &other.command_type {
                match &other.segment_type {
                    Segment::External(segment) => {
                        let key = command.clone().add(&"_").add(&segment);
                        let mut to_write = self
                            .push_pop_external_commands
                            .get(&key)
                            .unwrap()
                            .to_string();
                        let (mut segment, mut temp_address, mut static_address) =
                            (String::new(), 5, 16);
                        if segment == "static" {
                            static_address += other.index;
                            segment = self.file_name.to_string()
                        } else if segment == "temp" {
                            temp_address += other.index;
                            segment = "Temp".to_string();
                        } else if segment == "pointer" {
                            segment = "THIS".to_string();
                            if other.index == 1 {
                                segment = "THAT".to_string();
                            }
                        }
                        to_write = to_write
                            .replace("{i}", &other.index.clone().to_string())
                            .replace("{segment}", &segment)
                            .replace("{static}", &static_address.to_string())
                            .replace("{temp}", &temp_address.to_string());
                        writeln!(self.file, "{}", to_write).unwrap();
                    }
                    Segment::Internal(a) => {
                        let mut to_write = self
                            .push_pop_internal_commands
                            .get(&command.to_string())
                            .unwrap()
                            .to_string();
                        let mut segment: String = String::new();
                        if a == "local" {
                            segment = "LCL".to_string();
                        } else if a == "argument" {
                            segment = "ARG".to_string()
                        } else if a == "this" {
                            segment = "THIS".to_string()
                        } else if a == "that" {
                            segment = "THAT".to_string()
                        }
                        to_write = to_write
                            .replace("{i}", &other.index.clone().to_string())
                            .replace("{segment}", &segment);
                        writeln!(self.file, "{}", to_write).unwrap();
                    }
                    _ => panic!("Segment {:?} is not segment.", &other.segment_type),
                }
            } else {
                panic!("Command {:?} is not command function.", &other.command_type);
            };
        }
    }
}
