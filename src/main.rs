mod vm_translator;
use std::fs::File;
use std::io::BufReader;
use vm_translator::{CodeWriter, CodeWriterClass, Command, ParserClass, ParserPublic};
fn main() {
    let file = File::open("test.txt");
    let to_pass = BufReader::new(file.unwrap());
    let mut parser: ParserClass = ParserClass::new(to_pass);
    let mut write: CodeWriterClass = CodeWriterClass::new("output.txt".to_string());

    loop {
        let break_or = parser.has_more_commands();
        if !break_or {
            break;
        }
        match parser.command_type {
            Command::Arithmetic(_) => {
                write.write_arithmetic(&parser);
            }
            Command::Function(_) => {
                write.write_push_pop(&parser);
            }
            _ => continue,
        }
    }
}
