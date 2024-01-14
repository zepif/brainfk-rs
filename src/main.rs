use std::fs;
mod bf_parser;

fn main() {
    let context = inkwell::context::Context::create();
    let module = context.create_module("brainfuck");
    let builder = context.create_builder();

    let file_path = "./../brainfck//in1.b";
    let bf_code = match fs:read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading brainfuck code from file {}: {}", file_path, err);
            return;
        }
    };
    
    match bf_parser::parse_bs_code(&bf_code) {
        Ok(ast) => llvm_backend::generate_llvm_ir(ast),
        Err(err) => {
            eprintln("Error parsing brainfuck code: {}", err);
            return;
        }
    };

    module.println_to_file("output.ll").unwrap();
}
