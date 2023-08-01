# VirtualMachineTranslator
There are 2 modules that i created to translate VM Commands to Hack Assembly commands a **Parser Module** and **Writer Module**.

## VM Parser Module
The VM Parser module is responsible for reading VM commands from an input file and extracting relevant information. It provides a public interface for parsing VM commands and supports the following operations:
1. Creating an instance of the parser.
2. Checking if there are more commands to be parsed from the input file.
3. Accessing the attributes of the parser, such as the current command type, memory segment type, and index.

### Create an instance
To create an instance of the VM parser, follow these steps:
```ruby
// Import necessary modules
use vm_translator::parser::{ParserPublic, ParserClass};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Open the input file and create a BufReader
    let input_file = File::open("input.vm").expect("Failed to open input file");
    let buf_reader = BufReader::new(input_file);

    // Create an instance of the parser
    let parser = ParserClass::new(buf_reader);
}
```

### Methods
The VM Parser module provides the ParserPublic trait, which includes the methods new and has_more_commands. To use these methods, you can follow these examples:
```ruby
    // Create an instance of the parser
    let mut parser = ParserClass::new(buf_reader);

    // Check if there are more commands to be parsed
    parser.has_more_commands()
```
has_more_command() will return **true** if there is still command available to translate, it also will set attributes command type, memory segment type, and index to the current command automatically. It also will return false if there is no command available (the pointer has reached to the end of line).

### Attributes
The ParserClass struct provides public attributes that can be accessed after parsing the VM commands. These attributes include command_type, segment_type, and index, which represent the type of the current command, the memory segment associated with the command (if applicable), and the index used in certain commands (e.g., push/pop operations).

Please note that the ParserClass is designed to be used in the context of the provided Command and Segment enums from the modules module. Make sure to import the Command and Segment enums correctly to use them with the parser.

For example:
```ruby 
    // Check if there are more commands to be parsed
    while parser.has_more_commands() {
        // Parse the next command and access its attributes
        let command_type = parser.command_type();
        let segment_type = parser.segment_type();
        let index = parser.index;

        // Process the command attributes as needed
        match command_type {
            Command::Function(function_name) => {
                // Do something with the function name
                println!("Function command: {}", function_name);
            }
            Command::Arithmetic(arithmetic_op) => {
                // Do something with the arithmetic operation
                println!("Arithmetic command: {}", arithmetic_op);
            }
            Command::None => {
                println!("No command found.");
            }
        }

        // Access memory segment type and index (if applicable)
        match segment_type {
            Segment::Internal(segment_name) => {
                // Do something with the internal segment name and index
                println!("Internal segment: {} at index {}", segment_name, index);
            }
            Segment::External(segment_name) => {
                // Do something with the external segment name and index
                println!("External segment: {} at index {}", segment_name, index);
            }
            Segment::None => {
                println!("No memory segment found.");
            }
        }
    }
}
```

Please ensure that you have the correct file path for the input VM file and handle potential errors when opening the file. The above examples showcase the basic usage of the VM Parser module to read and process VM commands from the input file. Feel free to incorporate this module into your VM translator project to achieve your desired functionality.

## VM Code Writer Module
The VM Code Writer module is responsible for translating VM commands into assembly code and writing them to an output file. It provides a public interface for writing VM commands and supports the following operations:

1. Creating an instance of the code writer that writes to the specified output file.
2. Writing an arithmetic operation command to the output file.
3. Writing a push or pop command to the output file.

### How to Create an Instance
To create an instance of the VM code writer, follow these steps:
```ruby
use vm_translator::code_writer::{CodeWriter, CodeWriterClass};
use std::fs::File;

fn main() {
    // Specify the output file path where the translated assembly code will be written.
    let output_file = "output.asm".to_string();

    // Create an instance of the code writer
    let mut code_writer = CodeWriterClass::new(output_file);
}
```

### Methods
The VM Code Writer module provides the CodeWriter trait, which includes the methods new, write_arithmetic, and write_push_pop. To use these methods, you can follow these examples:
```ruby
use vm_translator::code_writer::{CodeWriter, CodeWriterClass};
use vm_translator::parser::ParserClass;
use vm_translator::modules::{Command, Segment};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Open the input file and create a BufReader
    let input_file = File::open("input.vm").expect("Failed to open input file");
    let buf_reader = BufReader::new(input_file);

    // Create an instance of the parser
    let mut parser = ParserClass::new(buf_reader);

    // Specify the output file path where the translated assembly code will be written.
    let output_file = "output.asm".to_string();

    // Create an instance of the code writer
    let mut cwriter = CodeWriterClass::new(output_file);

    // Check if there are more commands to be parsed
    loop {
        let break_or = parser.has_more_commands();
        if !break_or {
            break;
        }
        match parser.command_type {
        // CodeWriter instance module will automatically get necessary informations from Parser instance.
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
```

Please ensure that you have the correct file path for the output ASM file. The above examples showcase the basic usage of the VM Code Writer module to translate and write VM commands
