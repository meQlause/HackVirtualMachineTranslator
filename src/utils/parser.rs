use crate::prelude::*;
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

    /// The next VM command instruction to be processed.
    next_instruction: String,

    /// List of Commands supported VM arithmetic commands for parsing.
    arithmetic_commands: CommandList<String>,

    /// List of Commands supported VM push and pop commands for parsing.
    push_pop_commands: CommandList<String>,

    /// List of Commands supported VM branch commands for parsing.
    branch_commands: CommandList<String>,

    /// List of Commands supported VM function commands for parsing.
    function_commands: CommandList<String>,

    /// The current VM command being processed.
    pub current_command: String,

    /// The type of the current VM command.
    pub command_type: Option<Command>,

    /// The type of the memory segment associated with the current VM command (if applicable).
    pub segment_type: Option<Segment>,

    /// The index or offset used in VM commands that require it (e.g., push/pop operations).
    pub index: Option<i32>,
}

impl ParserPublic for ParserClass {
    /// Creates a new instance of the ParserClass with the provided input file.
    ///
    /// # Arguments
    ///
    /// * `input_file` - A `BufReader<File>` containing the input file to be parsed.
    ///
    /// # Returns
    ///
    /// A new instance of the ParserClass.
    fn new(input_file: BufReader<File>) -> Self {
        #[rustfmt::skip]
        let aritmetic: CommandList<String> = CommandList::new(vec!["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not",]);
        let push_pop: CommandList<String> = CommandList::new(vec!["push", "pop"]);
        let branch: CommandList<String> = CommandList::new(vec!["label", "if-goto", "goto"]);
        let function: CommandList<String> = CommandList::new(vec!["function", "call", "return"]);

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
        // Clear the previous next_instruction to avoid any lingering data.
        self.next_instruction.clear();

        loop {
            // Attempt to read the next line from the input file.
            let bytes = self
                .file
                .read_line(&mut self.next_instruction)
                .unwrap_or(0usize);

            if bytes > 0 {
                // Split the line by '/' to remove comments and other unnecessary data.
                let to_verified: Vec<String> = self
                    .next_instruction
                    .clone()
                    .split('/')
                    .map(String::from)
                    .collect();

                // Trim the line and take the first part as the verified next instruction.
                self.next_instruction = to_verified[0].clone().trim().to_string();

                // Skip lines that are comments (start with '/').
                if self.next_instruction.chars().next().unwrap_or('/') == '/' {
                    continue;
                }
                // If a valid command is found, set current_command by invoke advance() method with nextinstruction and return true.
                self.advance();
                return true;
            } else {
                // If no bytes are read, the end of the file is reached, so return false.
                return false;
            };
        }
    }
}

impl ParserPrivate for ParserClass {
    /// Advances the parser to the next command.
    ///
    /// This method updates the `current_command` field with the value of `next_instruction`,
    /// and then determines the type of the command and the associated segment type (if applicable).
    /// For push and pop commands, the `segment_type` field will be set accordingly.
    fn advance(&mut self) {
        // Update current_command with the next_instruction.
        self.current_command = self.next_instruction.clone();

        // Determine the type of the current command.
        self.command_type = self.command_type();

        // For push and pop commands, determine the segment type.
        if let Some(Command::PushPop(_)) = self.command_type {
            self.segment_type = self.segment_type();
        }
    }

    /// Determines the type of the current command.
    ///
    /// This method parses the `current_command` to identify its type.
    /// It checks if the command is one of the recognized arithmetic, push, pop, branch, or function commands.
    /// For push and pop commands, it extracts the segment index and sets the `index` field accordingly.
    ///
    /// # Returns
    ///
    /// * `Some(Command)` if the current command is recognized and its type is identified.
    /// * `None` if the current command is not recognized or is empty.
    fn command_type(&mut self) -> Option<Command> {
        // Split the current_command into parts.
        let a: Vec<&str> = self.current_command.split(' ').collect();

        if !self.current_command.is_empty() {
            match a[0].to_lowercase().trim() {
                command if self.push_pop_commands.is_exist(&command.to_string()) => {
                    // For push and pop commands, extract the segment index.
                    self.index = Some(a[2].trim().parse::<i32>().unwrap());
                    return Some(Command::PushPop(command.to_string()));
                }
                command if self.arithmetic_commands.is_exist(&command.to_string()) => {
                    // For arithmetic commands, there is no associated index.
                    self.index = None;
                    return Some(Command::Arithmetic(command.to_string()));
                }
                command if self.branch_commands.is_exist(&command.to_string()) => {
                    // For branch commands, there is no associated index.
                    self.index = None;
                    return Some(Command::Branch(command.to_string()));
                }
                command if self.function_commands.is_exist(&command.to_string()) => {
                    // For function commands, there is no associated index.
                    self.index = None;
                    return Some(Command::Function(command.to_string()));
                }
                _ => {
                    // If the command is not recognized, set the index to None and return None.
                    self.index = None;
                    return None;
                }
            };
        }
        // If the current command is empty, return None.
        None
    }

    /// Determines the segment type for push and pop commands.
    ///
    /// This method parses the `current_command` to extract the segment type.
    /// It checks if the segment type is one of the recognized internal or external segments.
    ///
    /// # Returns
    ///
    /// * `Some(Segment)` if the segment type is recognized and identified.
    /// * `None` if the segment type is not recognized or if it is not a push or pop command.
    fn segment_type(&self) -> Option<Segment> {
        // Split the current_command into parts.
        let a: Vec<&str> = self.current_command.split(' ').collect();

        // Define vectors of recognized internal and external segments.
        let internal: Vec<String> = vec!["local", "argument", "this", "that"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let external: Vec<String> = vec!["constant", "static", "temp", "pointer"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        match a[1].to_lowercase().trim() {
            segment if internal.contains(&segment.to_string()) => {
                // For internal segments, return the corresponding variant of the Segment enum.
                return Some(Segment::Internal(segment.to_string()));
            }
            segment if external.contains(&segment.to_string()) => {
                // For external segments, return the corresponding variant of the Segment enum.
                return Some(Segment::External(segment.to_string()));
            }
            _ => return None, // If the segment type is not recognized, return None.
        };
    }
}
