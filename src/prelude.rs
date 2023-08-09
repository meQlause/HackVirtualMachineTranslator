use std::{collections::HashMap, fmt::Display, hash::Hash};

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
pub struct List<T>(pub Vec<T>);

impl<T> List<T>
where
    T: Display + Into<String> + PartialEq,
{
    pub fn new(list: Vec<T>) -> List<String> {
        let t = list.into_iter().map(|x| x.to_string()).collect();
        List(t)
    }
    pub fn is_exist(&self, key: &T) -> bool {
        self.0.contains(key)
    }
}

// General data type for strong command
#[derive(Debug)]
pub struct CommandList<T>(pub HashMap<T, T>);

impl<T> CommandList<T>
where
    T: Display + Into<String> + PartialEq + Eq + Hash,
{
    pub fn new(list: Vec<(T, T)>) -> CommandList<String> {
        let t = list
            .into_iter()
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect();
        CommandList(t)
    }
    pub fn get(&self, key: &T) -> &T {
        self.0.get(key).unwrap()
    }
}

#[derive(Debug)]
pub struct State(pub Vec<i32>);

impl State {
    pub fn new() -> State {
        State(vec![0, 1])
    }

    pub fn get_logical(&self) -> String {
        self.0[0].to_string()
    }

    pub fn get_function(&self) -> String {
        self.0[1].to_string()
    }

    pub fn inc_logical(&mut self) {
        self.0[0] += 1;
    }

    pub fn inc_function(&mut self) {
        self.0[1] += 1;
    }
}
