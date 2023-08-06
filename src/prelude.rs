use std::fmt::Display;

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
            (Command::Function(_), Command::Function(_)) => true,
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

// General data type for strong command
#[derive(Debug)]
pub struct CommandList<T>(pub Vec<T>);

impl<T> CommandList<T>
where
    T: Display + Into<String> + PartialEq,
{
    pub fn new(list: Vec<T>) -> CommandList<String> {
        let t = list.into_iter().map(|x| x.to_string()).collect();
        CommandList(t)
    }
    pub fn is_exist(&self, key: &T) -> bool {
        self.0.contains(key)
    }
}
