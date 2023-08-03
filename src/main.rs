mod vm_translator;
use std::fs::File;
use std::io::BufReader;
use vm_translator::code_writer::{CodeWriter, CodeWriterClass};
use vm_translator::modules::Command;
use vm_translator::parser::{ParserClass, ParserPublic};
fn main() {
    let file = File::open("stackTest.vm");
    let to_pass = BufReader::new(file.unwrap());
    let mut parser: ParserClass = ParserClass::new(to_pass);
    let mut write: CodeWriterClass = CodeWriterClass::new("BasicLoop.asm".to_string());
    loop {
        let break_or = parser.has_more_commands();
        if !break_or {
            break;
        }
        match parser.command_type {
            Some(Command::Arithmetic(_)) => {
                write.write_arithmetic(&parser);
            }
            Some(Command::PushPop(_)) => {
                write.write_push_pop(&parser);
            }
            Some(Command::Branch(_)) => {
                write.write_branch(&parser);
            }
            _ => continue,
        }
    }
}
