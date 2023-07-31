mod lib;
use lib::{CodeWriter, CodeWriterClass, Parser, ParserClass};
use std::fs::File;
use std::io::BufReader;
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
        write.write_arithmetic(&parser);
        write.write_push_pop(&parser);
    }
}
