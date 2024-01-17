mod ast;
mod lexer;
mod parser;
mod token;

use ast::AST;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let cli = Cli::parse();

    let compilation_paths = CompilationPaths::new(&cli.input_path, &output_path, !cli.keep_files);

    let source = fs::read_to_string(&compilation_paths.source_path).expect(&format!(
        "Failed to read source {}",
        compilation_paths.source_path.to_str().unwrap()
    ));

    let lexer = Lexer::new(source);
    let ast = Parser::parse(lexer);

    if cli.dump_ast {
        println!("{:#?}", ast);
        return;
    }
}
