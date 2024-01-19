use clap::Parser;

/// Brainfuck to x86_64 assembly
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Brainfuck source file
    pub input_path: String,

    /// Output path
    #[arg(short = 'o', long)]
    pub output_path: Option<String>,

    /// Enable optimizations
    #[arg(short = 'O')]
    pub optimizations: bool,

    /// Output generated assembly
    #[arg(short = 'S', long)]
    pub assembly: bool,

    /// Keep intermediate files
    #[arg(long)]
    pub keep_files: bool,

    /// Print generated AST
    #[arg(long = "ast")]
    pub dump_ast: bool,
}
