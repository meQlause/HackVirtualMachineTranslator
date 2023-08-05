mod vm_translator;
use std::env;
use std::fs;
use std::io::BufReader;
use vm_translator::code_writer::{CodeWriter, CodeWriterClass};
use vm_translator::modules::Command;
use vm_translator::parser::{ParserClass, ParserPublic};

/// Main function of the VM Translator program.
///
/// This function serves as the entry point of the VM Translator executable. It takes command-line arguments
/// and performs the translation of VM code to assembly code for the target architecture. The program expects
/// either a single VM file or a directory containing multiple VM files as input. The output file will contain
/// the translated assembly code.
///
/// # Arguments
///
/// There are no direct arguments to the main function. Instead, the function reads command-line arguments
/// using `env::args()` and processes them accordingly.
///
/// # Example
///
/// ```
/// // Run the VM Translator with the following command-line arguments:
/// // vmtranslator.exe MyFile.vm MyOutput.asm
/// main();
/// ```
fn main() {
    // Retrieve command-line arguments into a vector of strings called `path`.
    let path: Vec<String> = env::args().collect();

    // Check if there are any command-line arguments (other than the program name itself).
    if path.len() > 1 {
        // Create a mutable instance of `CodeWriterClass` with the output file path.
        let mut writer: CodeWriterClass = CodeWriterClass::new(path[2].to_string());

        // Write the initialization code to the output file.
        writer.write_init();

        // Check if the second argument ends with the `.vm` extension.
        if path[1][path[1].len() - 3..].to_lowercase() == ".vm" {
            // Process the single VM file and write its assembly code to the output file.
            write_file(&path[1], &mut writer);
        } else if let Ok(entries) = fs::read_dir(&path[1]) {
            // If the second argument is a directory, read its entries and process VM files.
            for entry in entries {
                let file_path = entry.unwrap().path();
                if file_path.is_file() {
                    // Get the file path as a string.
                    let file = &file_path.to_str().unwrap().to_string();

                    // Update the `file_name` field of the `writer` with the current file name.
                    writer.file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

                    // Check if the file ends with the `.vm` extension.
                    if &file[file.len() - 3..] == ".vm" {
                        // Print a message indicating the current file is being executed.
                        println!("{:?} executed", file);

                        // Process the current VM file and write its assembly code to the output file.
                        write_file(&file, &mut writer);
                    }
                }
            }
        } else {
            // If the second argument is not a VM file or valid directory, print an error message.
            println!("File must be .vm / Directory not found");
        }
    } else {
        // Print usage message for the VM Translator executable.
        println!("Example: vmtranslator.exe <file(.vm extension) / Directory> <output.file>");
    }
}

fn write_file(input: &str, writer: &mut CodeWriterClass) {
    // Open the VM file specified by `input` and create a buffered reader (`to_pass`).
    let file = fs::File::open(input).expect("Cannot open file");
    let to_pass = BufReader::new(file);

    // Create a mutable instance of `ParserClass` and initialize it with the buffered reader.
    let mut parser: ParserClass = ParserClass::new(to_pass);

    // Loop until there are no more VM commands to process in the file.
    loop {
        let break_or = parser.has_more_commands();
        if !break_or {
            // If there are no more commands, exit the loop.
            break;
        }

        // Match the type of the current VM command and call the appropriate method on `writer`.
        match parser.command_type {
            Some(Command::Arithmetic(_)) => {
                // For arithmetic commands, write the corresponding assembly code.
                writer.write_arithmetic(&parser);
            }
            Some(Command::PushPop(_)) => {
                // For push/pop commands, write the corresponding assembly code.
                writer.write_push_pop(&parser);
            }
            Some(Command::Branch(_)) => {
                // For branch commands, write the corresponding assembly code.
                writer.write_branch(&parser);
            }
            Some(Command::Function(_)) => {
                // For function commands, write the corresponding assembly code.
                writer.write_function(&parser);
            }
            _ => continue,
        }
    }
}
