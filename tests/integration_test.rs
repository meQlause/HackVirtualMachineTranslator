// use std::fs::File;
// use std::io::{BufReader, Cursor, Seek};
// use virtual_machine_translator::{
//     CodeWriter, CodeWriterClass, Command, Parser, ParserClass, Segment,
// };

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn cursor_to_file(cursor: &mut Cursor<&str>) -> File {
//         let mut file = tempfile::tempfile().unwrap();
//         std::io::copy(cursor, &mut file).unwrap();
//         file.seek(std::io::SeekFrom::Start(0))
//             .expect("Failed to seek to the beginning of the file");
//         file
//     }

//     fn create_parser() -> ParserClass {
//         let mut text = Cursor::new("// test\npush static 1 //test\npop temp 2\n\nadd ");
//         let file = cursor_to_file(&mut text);
//         let input_file = BufReader::new(file);
//         ParserClass::new(input_file)
//     }

//     #[test]
//     fn test_has_more_commands() {
//         let mut parser = create_parser();
//         assert_eq!(parser.has_more_commands(), true);
//         assert_eq!(parser.has_more_commands(), true);
//         assert_eq!(parser.has_more_commands(), true);
//         assert_eq!(parser.has_more_commands(), false);
//     }

//     #[test]
//     fn test_advance() {
//         let mut parser = create_parser();
//         parser.advance();
//         assert_eq!(parser.command_type, Command::Function("push".to_string()));
//         assert_eq!(
//             parser.segment_type,
//             Segment::Eksternal("static".to_string())
//         );
//         assert_eq!(parser.index, 1);
//         parser.advance();
//         assert_eq!(parser.command_type, Command::Function("pop".to_string()));
//         assert_eq!(parser.segment_type, Segment::Eksternal("temp".to_string()));
//         assert_eq!(parser.index, 2);
//         parser.advance();
//         assert_eq!(parser.command_type, Command::Arithmetic("add".to_string()));
//         assert_eq!(parser.segment_type, Segment::None);
//         assert_eq!(parser.index, -1);
//     }

//     #[test]
//     fn test_command_type() {
//         let mut parser = create_parser();
//         parser.advance();
//         assert_eq!(parser.command_type, Command::Function("push".to_string()));
//         parser.advance();
//         assert_eq!(parser.command_type, Command::Function("pop".to_string()));
//         parser.advance();
//         assert_eq!(
//             parser.command_type(),
//             Command::Arithmetic("add".to_string())
//         );
//     }

//     #[test]
//     fn test_arg1() {
//         let mut parser = create_parser();
//         parser.advance();
//         assert_eq!(
//             parser.segment_type,
//             Segment::Eksternal("static".to_string())
//         );
//         parser.advance();
//         assert_eq!(parser.segment_type, Segment::Eksternal("temp".to_string()));
//     }

//     #[test]
//     fn test_arg2() {
//         let mut parser = create_parser();
//         parser.advance();
//         assert_eq!(parser.index, 1);
//         parser.advance();
//         assert_eq!(parser.index, 2);
//     }

//     fn create_write_instance() -> CodeWriterClass {
//         CodeWriterClass::new("output.txt".to_string())
//     }

//     #[test]
//     fn test_write_arithmetic() {
//         let object: CodeWriterClass = create_write_instance();
//         assert_eq!(object.write_arithmetic(Command::Arithmetic("eq".to_string())), "// eq@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@CON_TRUE_1\nD;JEQ\nM=0\n@CON_FINISH_1\n0;JMP\n(CON_TRUE_1)\nM=-1\n(CON_FINISH_1)\n@SP\nM=M+1\n".to_string());
//         assert_eq!(object.write_arithmetic(Command::Arithmetic("gt".to_string())), "// gt\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@CON_TRUE_1\nD;JGT\nM=0\n@CON_FINISH_1\n0;JMP\n(CON_TRUE_1)\nM=-1\n(CON_FINISH_1)\n@SP\nM=M+1\n".to_string());
//         assert_eq!(object.write_arithmetic(Command::Arithmetic("lt".to_string())), "// lt\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@CON_TRUE_1\nD;JLT\nM=0\n@CON_FINISH_1\n0;JMP\n(CON_TRUE_1)\nM=-1\n(CON_FINISH_1)\n@SP\nM=M+1\n".to_string());

//         assert_eq!(
//             object.write_arithmetic(Command::Arithmetic("addd".to_string())),
//             "//".to_string()
//         );
//         assert_eq!(
//             object.write_arithmetic(Command::Arithmetic("adddx".to_string())),
//             "//".to_string()
//         );
//     }
// }
